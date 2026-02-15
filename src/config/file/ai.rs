//! AI configuration for LLM-based cloze generation.

use crate::models::{Model, Provider, ProviderType};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Provider type DTO for configuration file serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderTypeDto {
    OpenAI,
    Anthropic,
    DeepSeek,
    Gemini,
    Ollama,
    Perplexity,
    XAI,
}

impl From<ProviderType> for ProviderTypeDto {
    fn from(pt: ProviderType) -> Self {
        match pt {
            ProviderType::OpenAI => ProviderTypeDto::OpenAI,
            ProviderType::Anthropic => ProviderTypeDto::Anthropic,
            ProviderType::DeepSeek => ProviderTypeDto::DeepSeek,
            ProviderType::Gemini => ProviderTypeDto::Gemini,
            ProviderType::Ollama => ProviderTypeDto::Ollama,
            ProviderType::Perplexity => ProviderTypeDto::Perplexity,
            ProviderType::XAI => ProviderTypeDto::XAI,
        }
    }
}

impl From<ProviderTypeDto> for ProviderType {
    fn from(dto: ProviderTypeDto) -> Self {
        match dto {
            ProviderTypeDto::OpenAI => ProviderType::OpenAI,
            ProviderTypeDto::Anthropic => ProviderType::Anthropic,
            ProviderTypeDto::DeepSeek => ProviderType::DeepSeek,
            ProviderTypeDto::Gemini => ProviderType::Gemini,
            ProviderTypeDto::Ollama => ProviderType::Ollama,
            ProviderTypeDto::Perplexity => ProviderType::Perplexity,
            ProviderTypeDto::XAI => ProviderType::XAI,
        }
    }
}

/// Provider configuration for AI settings.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProviderConfig {
    /// Unique identifier for this provider
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,

    /// Unique name for this provider (e.g., "openai", "anthropic")
    pub name: String,

    /// Provider type
    pub provider_type: ProviderTypeDto,

    /// Base URL for the API
    pub base_url: Option<String>,

    /// API key
    pub api_key: Option<String>,
}

impl From<&ProviderConfig> for Provider {
    fn from(config: &ProviderConfig) -> Self {
        Provider::builder()
            .id(config.id)
            .name(config.name.clone())
            .provider_type(config.provider_type.into())
            .base_url(config.base_url.clone().unwrap_or_default())
            .api_key(config.api_key.clone().unwrap_or_default())
            .build()
    }
}

impl From<ProviderConfig> for Provider {
    fn from(config: ProviderConfig) -> Self {
        Provider::from(&config)
    }
}

/// Model configuration for AI settings.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct ModelConfig {
    /// Unique identifier for this model
    #[serde(default = "Uuid::new_v4")]
    pub id: Uuid,

    /// Unique name for this model (e.g., "gpt-4", "claude-3")
    pub name: String,

    /// Provider ID this model belongs to
    pub provider_id: Uuid,

    /// Model ID on the provider platform
    pub model_id: String,
}

impl From<&ModelConfig> for Model {
    fn from(config: &ModelConfig) -> Self {
        Model::builder()
            .id(config.id)
            .name(config.name.clone())
            .provider_id(config.provider_id)
            .model_id(config.model_id.clone())
            .build()
    }
}

impl From<ModelConfig> for Model {
    fn from(config: ModelConfig) -> Self {
        Model::from(&config)
    }
}

/// AI configuration for LLM settings.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct AiConfig {
    /// List of providers
    #[serde(default)]
    pub providers: Vec<ProviderConfig>,

    /// List of models
    #[serde(default)]
    pub models: Vec<ModelConfig>,

    /// Currently selected model ID for cloze generation
    #[serde(default)]
    pub selected_model_id: Option<Uuid>,
}
