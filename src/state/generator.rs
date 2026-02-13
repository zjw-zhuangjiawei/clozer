use crate::models::{Cloze, Meaning, Model, Provider, ProviderType, Word};
use crate::registry::{ModelRegistry, ProviderRegistry};
use rig::agent::Agent;
use rig::client::{self, CompletionClient};
use rig::completion::Prompt;
use rig::providers::anthropic;
use rig::providers::deepseek;
use rig::providers::gemini;
use rig::providers::ollama;
use rig::providers::openai;
use rig::providers::perplexity;
use rig::providers::xai;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub enum AgentWrapper {
    OpenAI(Agent<openai::responses_api::ResponsesCompletionModel>),
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    DeepSeek(Agent<deepseek::CompletionModel>),
    Gemini(Agent<gemini::CompletionModel>),
    Ollama(Agent<ollama::CompletionModel>),
    Perplexity(Agent<perplexity::CompletionModel>),
    XAI(Agent<xai::CompletionModel>),
}

#[derive(Debug, Clone)]
pub struct GeneratorState {
    pub provider_registry: ProviderRegistry,
    pub model_registry: ModelRegistry,
    pub selected_model_id: Uuid,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorState {
    pub fn new() -> Self {
        let mut provider_registry = ProviderRegistry::new();
        let mut model_registry = ModelRegistry::new();

        // Add sample DeepSeek provider
        let provider = Provider::builder()
            .name("DeepSeek".to_string())
            .provider_type(ProviderType::DeepSeek)
            .base_url("https://api.deepseek.com".to_string())
            .api_key("***REMOVED***".to_string())
            .build();

        let provider_id = provider.id;
        provider_registry.add(provider);

        // Add sample model
        let model = Model::builder()
            .name("DeepSeek Chat".to_string())
            .provider_id(provider_id)
            .model_id("deepseek-chat".to_string())
            .build();
        let model_id = model.id;
        model_registry.add(model);

        Self {
            provider_registry,
            model_registry,
            selected_model_id: model_id,
        }
    }

    pub fn generator(&self) -> Arc<Generator> {
        let model = self.model_registry.get(self.selected_model_id).unwrap();
        let provider = self.provider_registry.get(model.provider_id).unwrap();
        Arc::new(Generator::new(provider, model))
    }
}

#[derive(Clone)]
pub struct Generator {
    agent: AgentWrapper,
}

impl Generator {
    pub fn new(provider: &Provider, model: &Model) -> Self {
        let agent = match provider.provider_type {
            ProviderType::OpenAI => {
                let client = openai::Client::builder()
                    .api_key(&provider.api_key)
                    .base_url(provider.base_url.clone())
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::OpenAI(agent)
            }
            ProviderType::Anthropic => {
                let client = anthropic::Client::builder()
                    .api_key(&provider.api_key)
                    .base_url(provider.base_url.clone())
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::Anthropic(agent)
            }
            ProviderType::DeepSeek => {
                let client = deepseek::Client::builder()
                    .api_key(&provider.api_key)
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::DeepSeek(agent)
            }
            ProviderType::Gemini => {
                let client = gemini::Client::builder()
                    .api_key(&provider.api_key)
                    .base_url(provider.base_url.clone())
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::Gemini(agent)
            }
            ProviderType::Ollama => {
                let client = ollama::Client::builder()
                    .api_key(client::Nothing)
                    .base_url(provider.base_url.clone())
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::Ollama(agent)
            }
            ProviderType::Perplexity => {
                let client = perplexity::Client::builder()
                    .api_key(&provider.api_key)
                    .base_url(provider.base_url.clone())
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::Perplexity(agent)
            }
            ProviderType::XAI => {
                let client = xai::Client::builder()
                    .api_key(&provider.api_key)
                    .base_url(provider.base_url.clone())
                    .build()
                    .unwrap();
                let agent = client.agent(&model.model_id).build();
                AgentWrapper::XAI(agent)
            }
        };

        Self { agent }
    }

    pub async fn generate(&self, word: &Word, meaning: &Meaning) -> Cloze {
        tracing::debug!(
            "Generating cloze for word: {} ({})",
            word.content,
            meaning.pos
        );

        let prompt = format!(
            r#"Generate a cloze deletion sentence for "{content}" with definition "{definition}" ({pos}).
Use brackets to mark the blank: [answer]
Example: "The [cat] sat on the mat"
Return ONLY the sentence."#,
            content = word.content,
            definition = meaning.definition,
            pos = meaning.pos
        );

        let start = std::time::Instant::now();
        let sentence = match &self.agent {
            AgentWrapper::OpenAI(a) => a.prompt(&prompt).await.unwrap(),
            AgentWrapper::Anthropic(a) => a.prompt(&prompt).await.unwrap(),
            AgentWrapper::DeepSeek(a) => a.prompt(&prompt).await.unwrap(),
            AgentWrapper::Gemini(a) => a.prompt(&prompt).await.unwrap(),
            AgentWrapper::Ollama(a) => a.prompt(&prompt).await.unwrap(),
            AgentWrapper::Perplexity(a) => a.prompt(&prompt).await.unwrap(),
            AgentWrapper::XAI(a) => a.prompt(&prompt).await.unwrap(),
        };
        let elapsed = start.elapsed().as_millis();
        tracing::debug!("LLM request completed in {}ms", elapsed);

        let segments = Cloze::parse_from_sentence(&sentence);
        Cloze::builder()
            .meaning_id(meaning.id)
            .segments(segments)
            .build()
    }
}
