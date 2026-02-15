//! CLI tool to create a Clozer database from JSON file.

use clap::Parser;
use clozer::models::{Cloze, Meaning, PartOfSpeech, Tag, Word};
use clozer::persistence::db::Db;
use clozer::registry::{ClozeRegistry, MeaningRegistry, TagRegistry, WordRegistry};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Part of speech for JSON import.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum PartOfSpeechDto {
    Noun,
    Verb,
    Adjective,
    Adverb,
    Pronoun,
    Preposition,
    Conjunction,
    Interjection,
    Determiner,
    Article,
    Modal,
    Numeral,
    Abbreviation,
}

/// Import data structure from JSON file.
#[derive(Debug, Deserialize)]
struct ImportData {
    tags: Vec<TagDto>,
    words: Vec<WordDto>,
}

/// Tag DTO for JSON import.
#[derive(Debug, Deserialize)]
struct TagDto {
    name: String,
}

/// Word DTO for JSON import.
#[derive(Debug, Deserialize)]
struct WordDto {
    content: String,
    meanings: Vec<MeaningDto>,
}

/// Meaning DTO for JSON import.
#[derive(Debug, Deserialize)]
struct MeaningDto {
    definition: String,
    pos: PartOfSpeechDto,
    clozes: Vec<String>,
}

/// Create a Clozer database from JSON file.
#[derive(Parser, Debug)]
#[command(name = "create-sample-db")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to JSON file containing data to import
    json_file: PathBuf,
    /// Path to output database file
    db_path: PathBuf,
}

/// Statistics about loaded entities.
#[derive(Debug, Default)]
struct LoadStats {
    tags: usize,
    words: usize,
    meanings: usize,
    clozes: usize,
}

impl From<PartOfSpeechDto> for PartOfSpeech {
    fn from(pos: PartOfSpeechDto) -> Self {
        match pos {
            PartOfSpeechDto::Noun => PartOfSpeech::Noun,
            PartOfSpeechDto::Verb => PartOfSpeech::Verb,
            PartOfSpeechDto::Adjective => PartOfSpeech::Adjective,
            PartOfSpeechDto::Adverb => PartOfSpeech::Adverb,
            PartOfSpeechDto::Pronoun => PartOfSpeech::Pronoun,
            PartOfSpeechDto::Preposition => PartOfSpeech::Preposition,
            PartOfSpeechDto::Conjunction => PartOfSpeech::Conjunction,
            PartOfSpeechDto::Interjection => PartOfSpeech::Interjection,
            PartOfSpeechDto::Determiner => PartOfSpeech::Determiner,
            PartOfSpeechDto::Article => PartOfSpeech::Article,
            PartOfSpeechDto::Modal => PartOfSpeech::Modal,
            PartOfSpeechDto::Numeral => PartOfSpeech::Numeral,
            PartOfSpeechDto::Abbreviation => PartOfSpeech::Abbreviation,
        }
    }
}

/// Load data from JSON file and populate database.
fn load_from_json(json_path: &Path, db: &Db) -> Result<LoadStats, Box<dyn std::error::Error>> {
    let json_content = std::fs::read_to_string(json_path)?;
    let import_data: ImportData = serde_json::from_str(&json_content)?;

    let mut tag_registry = TagRegistry::new();
    let mut word_registry = WordRegistry::new();
    let mut meaning_registry = MeaningRegistry::new();
    let mut cloze_registry = ClozeRegistry::new();

    let mut stats = LoadStats::default();

    // Create tags
    for tag_dto in &import_data.tags {
        let tag = Tag::builder().name(tag_dto.name.clone()).build();
        tag_registry.add(tag);
        stats.tags += 1;
    }

    // Create words, meanings, and clozes
    for word_dto in &import_data.words {
        let mut word = Word::builder().content(word_dto.content.clone()).build();
        let word_id = word.id;
        let mut meaning_ids = Vec::new();

        for meaning_dto in &word_dto.meanings {
            let meaning = Meaning::builder()
                .word_id(word_id)
                .definition(meaning_dto.definition.clone())
                .pos(meaning_dto.pos.into())
                .build();
            let meaning_id = meaning.id;
            meaning_ids.push(meaning_id);

            // Create clozes for this meaning
            for sentence in &meaning_dto.clozes {
                let segments = Cloze::parse_from_sentence(sentence);
                let cloze = Cloze::builder()
                    .meaning_id(meaning_id)
                    .segments(segments)
                    .build();
                cloze_registry.add(cloze);
                stats.clozes += 1;
            }

            meaning_registry.add(meaning);
            stats.meanings += 1;
        }

        // Update word with meaning_ids
        for meaning_id in &meaning_ids {
            word.meaning_ids.insert(*meaning_id);
        }
        word_registry.add(word);
        stats.words += 1;
    }

    // Flush all registries to database
    tag_registry.flush_dirty(db)?;
    word_registry.flush_dirty(db)?;
    meaning_registry.flush_dirty(db)?;
    cloze_registry.flush_dirty(db)?;

    Ok(stats)
}

fn main() {
    let args = Args::parse();

    println!("Loading data from: {:?}", args.json_file);
    println!("Creating database at: {:?}", args.db_path);

    let db = match Db::new(&args.db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to create database: {}", e);
            return;
        }
    };

    match load_from_json(&args.json_file, &db) {
        Ok(stats) => {
            println!("\n=== Database Created ===");
            println!("Tags:       {}", stats.tags);
            println!("Words:      {}", stats.words);
            println!("Meanings:   {}", stats.meanings);
            println!("Clozes:     {}", stats.clozes);
            println!("\nDatabase path: {:?}", args.db_path);
        }
        Err(e) => {
            eprintln!("Failed to load JSON: {}", e);
        }
    }
}
