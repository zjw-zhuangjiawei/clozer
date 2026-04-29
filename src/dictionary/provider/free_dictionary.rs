use serde::Deserialize;

use super::error::DictionaryError;
use crate::dictionary::models::DictionaryEntry;

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

    Ok(DictionaryEntry { word: entry.word })
}

#[derive(Deserialize)]
struct ApiResponse {
    word: String,
}
