use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    #[builder(default = Uuid::new_v4())]
    pub id: Uuid,
    pub name: String,
    pub provider_type: ProviderType,
    pub base_url: String,
    pub api_key: String,
}
