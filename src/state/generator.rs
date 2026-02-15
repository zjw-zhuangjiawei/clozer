use crate::config::AiConfig;
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
    pub selected_model_id: Option<Uuid>,
}

impl Default for GeneratorState {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratorState {
    pub fn new() -> Self {
        Self {
            provider_registry: ProviderRegistry::new(),
            model_registry: ModelRegistry::new(),
            selected_model_id: None,
        }
    }

    /// Loads AI configuration (providers and models) into this state.
    pub fn load_from_config(&mut self, config: &AiConfig) {
        self.provider_registry.load_from_config(config);
        self.model_registry.load_from_config(config);

        // Use config's selected_model_id, or auto-select first model if none set
        if let Some(selected_id) = config.selected_model_id {
            // Validate the selected model exists
            if self.model_registry.get(selected_id).is_some() {
                self.selected_model_id = Some(selected_id);
            } else {
                tracing::warn!(
                    "Config selected_model_id {} not found in models, auto-selecting first",
                    selected_id
                );
                self.selected_model_id = self.model_registry.iter().next().map(|(id, _)| *id);
            }
        } else {
            // Auto-select first model if none selected
            self.selected_model_id = self.model_registry.iter().next().map(|(id, _)| *id);
        }

        tracing::debug!(
            "Loaded AI config: {} providers, {} models, selected_model_id={:?}",
            self.provider_registry.len(),
            self.model_registry.len(),
            self.selected_model_id,
        );
    }

    /// Selects a model by ID. Returns true if successful.
    pub fn select_model(&mut self, model_id: Uuid) -> bool {
        if self.model_registry.get(model_id).is_some() {
            self.selected_model_id = Some(model_id);
            true
        } else {
            false
        }
    }

    /// Gets the currently selected model, if any.
    pub fn selected_model(&self) -> Option<&Model> {
        self.selected_model_id.and_then(|id| self.model_registry.get(id))
    }

    pub fn generator(&self) -> Arc<Generator> {
        let model_id = self.selected_model_id.expect("No model selected");
        let model = self.model_registry.get(model_id).expect("Model not found");
        let provider = self
            .provider_registry
            .get(model.provider_id)
            .expect("Provider not found");
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
