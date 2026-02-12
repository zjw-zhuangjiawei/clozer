//! Provider DTO for serialization.

use crate::models::{Provider, ProviderType};
use serde::{Deserialize, Serialize};

// Provider type DTO for serialization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// Provider entity data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderDto {
    pub name: String,
    #[serde(rename = "type")]
    pub provider_type: ProviderTypeDto,
    pub base_url: String,
    pub api_key: String,
}

impl From<&Provider> for ProviderDto {
    fn from(provider: &Provider) -> Self {
        ProviderDto {
            name: provider.name.clone(),
            provider_type: provider.provider_type.into(),
            base_url: provider.base_url.clone(),
            api_key: provider.api_key.clone(),
        }
    }
}

impl From<ProviderDto> for Provider {
    fn from(dto: ProviderDto) -> Self {
        Provider::builder()
            .name(dto.name)
            .provider_type(dto.provider_type.into())
            .base_url(dto.base_url)
            .api_key(dto.api_key)
            .build()
    }
}
