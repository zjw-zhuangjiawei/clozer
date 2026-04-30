use serde::Deserialize;

use super::error::DictionaryError;
use crate::dictionary::models::{DictionaryDefinition, DictionaryEntry, DictionaryMeaning};

const BASE_URL: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

pub async fn lookup(word: &str) -> Result<DictionaryEntry, DictionaryError> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", BASE_URL, word);

    let response = client.get(&url).send().await?;

    if response.status() == 404 {
        return Err(DictionaryError::WordNotFound {
            word: word.to_string(),
        });
    }

    if !response.status().is_success() {
        return Err(DictionaryError::HttpStatus {
            status: response.status().as_u16(),
        });
    }

    let mut api_response: Vec<ApiResponse> = response.json().await?;
    if api_response.is_empty() {
        return Err(DictionaryError::UnexpectedFormat);
    }
    let entry = api_response.remove(0);

    let meanings = entry
        .meanings
        .into_iter()
        .map(|m| DictionaryMeaning {
            part_of_speech: m.part_of_speech,
            definitions: m
                .definitions
                .into_iter()
                .map(|d| DictionaryDefinition {
                    definition: d.definition,
                    example: d.example,
                })
                .collect(),
        })
        .collect();

    Ok(DictionaryEntry {
        word: entry.word,
        meanings,
    })
}

#[derive(Deserialize)]
struct ApiResponse {
    word: String,
    #[serde(default)]
    meanings: Vec<ApiMeaning>,
}

#[derive(Deserialize)]
struct ApiMeaning {
    #[serde(rename = "partOfSpeech")]
    part_of_speech: String,
    #[serde(default)]
    definitions: Vec<ApiDefinition>,
}

#[derive(Deserialize)]
struct ApiDefinition {
    definition: String,
    example: Option<String>,
}
