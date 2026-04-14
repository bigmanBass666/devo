//! Runtime model types shared across core, server, and clients.
//!
//! Main focus:
//! - represent the resolved model shape used during execution
//! - expose model capabilities needed by UI, request building, and turn resolution
//! - provide the read-only catalog trait over runtime `Model` values
//!
//! Design:
//! - `Model` is the cross-crate runtime type, not the raw config/catalog input type
//! - this module keeps behavior that belongs to the executable model itself, such as
//!   thinking resolution and effective defaults
//! - callers should be able to use this type without knowing how the model catalog was loaded
//!
//! Boundary:
//! - this module must not own bundled JSON loading or compatibility parsing for catalog files
//! - raw preset/config concerns live in `clawcr-core`
//! - this module describes runtime state and runtime-facing interfaces only
//!
use clawcr_provider::ProviderFamily;
use serde::{Deserialize, Serialize};

use crate::{
    ReasoningEffort, ReasoningEffortPreset, ResolvedThinkingRequest, ThinkingCapability,
    ThinkingImplementation, nearest_effort, truncation::TruncationPolicyConfig,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Verbosity {
    Low,
    Medium,
    High,
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
/// Supported input types that a model can accept.
pub enum InputModality {
    /// Plain text input.
    Text,
    /// Image input.
    Image,
}

impl Default for InputModality {
    fn default() -> Self {
        Self::Text
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
/// Resolved runtime model metadata used across core, server, and clients.
pub struct Model {
    /// Stable model identifier used in config and requests. such as `claude-sonnet-20250425`
    pub slug: String,
    /// Human-readable display name shown in the UI. such as `claude-sonnet-4.6`
    pub display_name: String,
    /// Provider family that serves this model.
    pub provider_family: ProviderFamily,
    /// Optional short description of the model.
    pub description: Option<String>,
    /// Thinking control available for this model.
    pub thinking_capability: ThinkingCapability,
    /// Default reasoning effort selected for the model when no levels are exposed.
    pub default_reasoning_effort: Option<ReasoningEffort>,
    /// How the selected thinking mode should be applied to requests.
    pub thinking_implementation: Option<ThinkingImplementation>,
    /// Base system instructions bundled with the model.
    pub base_instructions: String,
    /// Maximum context window in tokens.
    pub context_window: u32,
    /// Percentage of the context window treated as effectively usable.
    pub effective_context_window_percent: Option<u8>,
    /// Policy used when truncating content for requests.
    pub truncation_policy: TruncationPolicyConfig,
    /// Input types accepted by the model.
    pub input_modalities: Vec<InputModality>,
    /// Whether the model supports original-resolution image detail.
    pub supports_image_detail_original: bool,
    /// Default temperature to use when the model does not override it.
    pub temperature: Option<f32>,
    /// Default nucleus sampling value to use when the model does not override it.
    pub top_p: Option<f32>,
    /// Default top-k sampling value to use when the model does not override it.
    pub top_k: Option<f32>,
    /// Default maximum token limit for responses from this model.
    pub max_tokens: Option<u32>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            slug: String::new(),
            display_name: String::new(),
            provider_family: ProviderFamily::OpenAI,
            description: None,
            thinking_capability: ThinkingCapability::Disabled,
            default_reasoning_effort: Some(ReasoningEffort::default()),
            thinking_implementation: None,
            base_instructions: String::new(),
            context_window: 200_000,
            effective_context_window_percent: None,
            truncation_policy: TruncationPolicyConfig::default(),
            input_modalities: vec![InputModality::default()],
            supports_image_detail_original: false,
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens: None,
        }
    }
}

impl Model {
    pub fn reasoning_effort_options(&self) -> Vec<ReasoningEffortPreset> {
        match &self.thinking_capability {
            ThinkingCapability::Levels(levels) => levels
                .iter()
                .copied()
                .map(|effort| ReasoningEffortPreset::new(effort, effort.description()))
                .collect(),
            _ => self
                .default_reasoning_effort
                .iter()
                .copied()
                .map(|effort| ReasoningEffortPreset::new(effort, effort.description()))
                .collect(),
        }
    }

    pub fn effective_thinking_capability(&self) -> ThinkingCapability {
        self.thinking_capability.clone()
    }

    pub fn effective_thinking_implementation(&self) -> ThinkingImplementation {
        self.thinking_implementation.clone().unwrap_or_else(|| {
            if matches!(self.thinking_capability, ThinkingCapability::Disabled) {
                ThinkingImplementation::Disabled
            } else {
                ThinkingImplementation::RequestParameter
            }
        })
    }

    pub fn effective_context_window_percent(&self) -> u8 {
        self.effective_context_window_percent.unwrap_or(95)
    }

    pub fn default_thinking_selection(&self) -> Option<String> {
        match &self.thinking_capability {
            ThinkingCapability::Disabled => None,
            ThinkingCapability::Toggle => Some(String::from("enabled")),
            ThinkingCapability::Levels(levels) => self
                .default_reasoning_effort
                .or_else(|| levels.first().copied())
                .map(|effort| effort.label().to_lowercase()),
        }
    }

    pub fn nearest_supported_reasoning_effort(&self, target: ReasoningEffort) -> ReasoningEffort {
        match &self.thinking_capability {
            ThinkingCapability::Levels(levels) if !levels.is_empty() => {
                nearest_effort(target, levels)
            }
            _ => self.default_reasoning_effort.unwrap_or(target),
        }
    }

    pub fn resolve_thinking_selection(&self, selection: Option<&str>) -> ResolvedThinkingRequest {
        let normalized_selection = selection
            .map(str::trim)
            .filter(|selection| !selection.is_empty())
            .map(|selection| selection.to_ascii_lowercase())
            .or_else(|| self.default_thinking_selection());

        match self.effective_thinking_implementation() {
            ThinkingImplementation::Disabled => ResolvedThinkingRequest {
                request_model: self.slug.clone(),
                request_thinking: None,
                effective_reasoning_effort: None,
                extra_body: None,
            },
            ThinkingImplementation::RequestParameter => {
                let request_thinking = match self.effective_thinking_capability() {
                    ThinkingCapability::Disabled => None,
                    ThinkingCapability::Toggle => normalized_selection
                        .filter(|selection| selection == "enabled" || selection == "disabled"),
                    ThinkingCapability::Levels(_) => normalized_selection.map(|selection| {
                        let parsed = selection
                            .parse::<ReasoningEffort>()
                            .ok()
                            .map(|effort| self.nearest_supported_reasoning_effort(effort))
                            .unwrap_or_else(|| self.default_reasoning_effort.unwrap_or_default());
                        parsed.label().to_lowercase()
                    }),
                };
                let effective_reasoning_effort = request_thinking
                    .as_deref()
                    .and_then(|selection| selection.parse::<ReasoningEffort>().ok())
                    .or(self.default_reasoning_effort);
                ResolvedThinkingRequest {
                    request_model: self.slug.clone(),
                    request_thinking,
                    effective_reasoning_effort,
                    extra_body: None,
                }
            }
            ThinkingImplementation::ModelVariant(config) => {
                let selected_variant = normalized_selection
                    .as_deref()
                    .and_then(|selection| {
                        config
                            .variants
                            .iter()
                            .find(|variant| variant.selection_value.eq_ignore_ascii_case(selection))
                    })
                    .or_else(|| {
                        self.default_thinking_selection()
                            .as_deref()
                            .and_then(|selection| {
                                config.variants.iter().find(|variant| {
                                    variant.selection_value.eq_ignore_ascii_case(selection)
                                })
                            })
                    })
                    .or_else(|| config.variants.first());
                if let Some(variant) = selected_variant {
                    ResolvedThinkingRequest {
                        request_model: variant.model_slug.clone(),
                        request_thinking: None,
                        effective_reasoning_effort: variant.reasoning_effort,
                        extra_body: None,
                    }
                } else {
                    ResolvedThinkingRequest {
                        request_model: self.slug.clone(),
                        request_thinking: None,
                        effective_reasoning_effort: self.default_reasoning_effort,
                        extra_body: None,
                    }
                }
            }
        }
    }
}

/// Provides read-only access to resolved runtime model definitions.
pub trait ModelCatalog: Send + Sync {
    fn list_visible(&self) -> Vec<&Model>;
    fn get(&self, slug: &str) -> Option<&Model>;
    fn resolve_for_turn(&self, requested: Option<&str>) -> Result<&Model, ModelError>;
}

#[derive(Debug, Clone)]
pub struct InMemoryModelCatalog {
    models: Vec<Model>,
}

impl InMemoryModelCatalog {
    pub fn new(models: Vec<Model>) -> Self {
        Self { models }
    }
}

impl ModelCatalog for InMemoryModelCatalog {
    fn list_visible(&self) -> Vec<&Model> {
        self.models.iter().collect()
    }

    fn get(&self, slug: &str) -> Option<&Model> {
        self.models.iter().find(|model| model.slug == slug)
    }

    fn resolve_for_turn(&self, requested: Option<&str>) -> Result<&Model, ModelError> {
        if let Some(slug) = requested {
            return self.get(slug).ok_or_else(|| ModelError::ModelNotFound {
                slug: slug.to_string(),
            });
        }

        self.list_visible()
            .into_iter()
            .next()
            .ok_or(ModelError::NoVisibleModels)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ModelError {
    #[error("model not found: {slug}")]
    ModelNotFound { slug: String },
    #[error("no visible models available")]
    NoVisibleModels,
}

#[cfg(test)]
mod tests {
    use crate::{ThinkingVariant, ThinkingVariantConfig};
    use pretty_assertions::assert_eq;

    use super::{
        InMemoryModelCatalog, InputModality, Model, ModelCatalog, ProviderFamily, ReasoningEffort,
        ThinkingCapability, ThinkingImplementation, TruncationPolicyConfig,
    };

    fn model(slug: &str) -> Model {
        Model {
            slug: slug.into(),
            display_name: slug.into(),
            provider_family: ProviderFamily::Anthropic,
            description: None,
            thinking_capability: ThinkingCapability::Disabled,
            default_reasoning_effort: Some(ReasoningEffort::Medium),
            thinking_implementation: None,
            base_instructions: String::new(),
            context_window: 200_000,
            effective_context_window_percent: None,
            truncation_policy: TruncationPolicyConfig {
                mode: crate::TruncationMode::Tokens,
                limit: 10000,
            },
            input_modalities: vec![InputModality::Text],
            supports_image_detail_original: false,
            temperature: None,
            top_p: None,
            top_k: None,
            max_tokens: None,
        }
    }

    #[test]
    fn resolve_for_turn_honors_requested_slug() {
        let catalog = InMemoryModelCatalog::new(vec![model("test")]);
        let resolved = catalog
            .resolve_for_turn(Some("test"))
            .expect("resolve explicit");
        assert_eq!(resolved.slug, "test");
    }

    #[test]
    fn resolve_thinking_selection_disables_request_thinking_when_capability_is_disabled() {
        let preset = model("test");

        let resolved = preset.resolve_thinking_selection(Some("enabled"));

        assert_eq!(resolved.request_model, "test");
        assert_eq!(resolved.request_thinking, None);
        assert_eq!(resolved.effective_reasoning_effort, None);
    }

    #[test]
    fn resolve_thinking_selection_uses_request_parameter_for_toggle_models() {
        let mut preset = model("glm-5.1");
        preset.thinking_capability = ThinkingCapability::Toggle;

        let resolved = preset.resolve_thinking_selection(Some("disabled"));

        assert_eq!(resolved.request_model, "glm-5.1");
        assert_eq!(resolved.request_thinking, Some(String::from("disabled")));
        assert_eq!(
            resolved.effective_reasoning_effort,
            Some(ReasoningEffort::Medium)
        );
    }

    #[test]
    fn resolve_thinking_selection_snaps_effort_for_level_models() {
        let mut preset = model("o-model");
        preset.thinking_capability =
            ThinkingCapability::Levels(vec![ReasoningEffort::Low, ReasoningEffort::High]);
        preset.default_reasoning_effort = Some(ReasoningEffort::Low);

        let resolved = preset.resolve_thinking_selection(Some("medium"));

        assert_eq!(resolved.request_model, "o-model");
        assert_eq!(resolved.request_thinking, Some(String::from("low")));
        assert_eq!(
            resolved.effective_reasoning_effort,
            Some(ReasoningEffort::Low)
        );
    }

    #[test]
    fn resolve_thinking_selection_uses_model_variants_when_configured() {
        let mut preset = model("kimi-k2.5");
        preset.thinking_capability = ThinkingCapability::Toggle;
        preset.thinking_implementation = Some(ThinkingImplementation::ModelVariant(
            ThinkingVariantConfig {
                variants: vec![
                    ThinkingVariant {
                        selection_value: String::from("disabled"),
                        model_slug: String::from("kimi-k2.5"),
                        reasoning_effort: None,
                        label: String::from("Off"),
                        description: String::from("Use the standard model"),
                    },
                    ThinkingVariant {
                        selection_value: String::from("enabled"),
                        model_slug: String::from("kimi-k2.5-thinking"),
                        reasoning_effort: Some(ReasoningEffort::Medium),
                        label: String::from("On"),
                        description: String::from("Use the thinking model"),
                    },
                ],
            },
        ));

        let resolved = preset.resolve_thinking_selection(Some("enabled"));

        assert_eq!(resolved.request_model, "kimi-k2.5-thinking");
        assert_eq!(resolved.request_thinking, None);
        assert_eq!(
            resolved.effective_reasoning_effort,
            Some(ReasoningEffort::Medium)
        );
    }

    #[test]
    fn resolve_thinking_selection_falls_back_to_first_variant_when_selection_is_invalid() {
        let mut preset = model("deepseek-chat");
        preset.thinking_capability = ThinkingCapability::Toggle;
        preset.thinking_implementation = Some(ThinkingImplementation::ModelVariant(
            ThinkingVariantConfig {
                variants: vec![ThinkingVariant {
                    selection_value: String::from("disabled"),
                    model_slug: String::from("deepseek-chat"),
                    reasoning_effort: None,
                    label: String::from("Off"),
                    description: String::from("Use the standard model"),
                }],
            },
        ));

        let resolved = preset.resolve_thinking_selection(Some("invalid"));

        assert_eq!(resolved.request_model, "deepseek-chat");
        assert_eq!(resolved.request_thinking, None);
    }
}
