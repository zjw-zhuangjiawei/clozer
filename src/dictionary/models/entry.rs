#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    pub word: String,
    pub meanings: Vec<DictionaryMeaning>,
}

#[derive(Debug, Clone)]
pub struct DictionaryMeaning {
    pub part_of_speech: String,
    pub definitions: Vec<DictionaryDefinition>,
}

#[derive(Debug, Clone)]
pub struct DictionaryDefinition {
    pub definition: String,
    pub example: Option<String>,
}
