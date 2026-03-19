use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use super::ProviderId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderType {
    OpenAI,
    Anthropic,
    DeepSeek,
    Gemini,
    Ollama,
    Perplexity,
    XAI,
}

#[derive(Debug, Clone, TypedBuilder)]
pub struct Provider {
    #[builder(default = ProviderId::new())]
    pub id: ProviderId,
    pub name: String,
    pub provider_type: ProviderType,
    pub base_url: String,
    pub api_key: String,
}
