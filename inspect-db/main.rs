//! CLI tool to inspect Clozer database files.

use clap::{Parser, ValueEnum};
use clozer::persistence::db::Db;
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum)]
enum TableName {
    Words,
    Meanings,
    Closes,
    Tags,
}

/// Inspect Clozer database files.
#[derive(Parser, Debug)]
#[command(name = "inspect-db")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the database file (required positional argument)
    #[arg(value_name = "DB_PATH")]
    db_path: PathBuf,

    /// Optional: specific table to inspect
    #[arg(short, long, value_name = "TABLE")]
    table: Option<TableName>,
}

fn print_words(db: &Db) {
    println!("=== WORDS ===");
    match db.iter_words() {
        Ok(words) => {
            for word in words {
                println!("id: {}", word.id);
                println!("content: \"{}\"", word.content);
                println!("meaning_ids: {:?}", word.meaning_ids);
                println!();
            }
        }
        Err(e) => eprintln!("Error reading words: {}", e),
    }
}

fn print_meanings(db: &Db) {
    println!("=== MEANINGS ===");
    match db.iter_meanings() {
        Ok(meanings) => {
            for meaning in meanings {
                println!("id: {}", meaning.id);
                println!("definition: \"{}\"", meaning.definition);
                println!("pos: {:?}", meaning.pos);
                println!("word_id: {}", meaning.word_id);
                println!("tag_ids: {:?}", meaning.tag_ids);
                println!("cloze_ids: {:?}", meaning.cloze_ids);
                println!();
            }
        }
        Err(e) => eprintln!("Error reading meanings: {}", e),
    }
}

fn print_clozes(db: &Db) {
    println!("=== CLOZES ===");
    match db.iter_clozes() {
        Ok(clozes) => {
            for cloze in clozes {
                println!("id: {}", cloze.id);
                println!("segments:");
                for segment in &cloze.segments {
                    match segment {
                        clozer::persistence::dto::ClozeSegmentDto::Text(s) => {
                            println!("  - Text: \"{}\"", s);
                        }
                        clozer::persistence::dto::ClozeSegmentDto::Blank(s) => {
                            println!("  - Blank: \"{}\"", s);
                        }
                    }
                }
                println!("meaning_id: {}", cloze.meaning_id);
                println!();
            }
        }
        Err(e) => eprintln!("Error reading clozes: {}", e),
    }
}

fn print_tags(db: &Db) {
    println!("=== TAGS ===");
    match db.iter_tags() {
        Ok(tags) => {
            for tag in tags {
                println!("id: {}", tag.id);
                println!("name: \"{}\"", tag.name);
                println!("parent_id: {:?}", tag.parent_id);
                println!("children_ids: {:?}", tag.children_ids);
                println!();
            }
        }
        Err(e) => eprintln!("Error reading tags: {}", e),
    }
}

fn main() {
    let args = Args::parse();

    let db = match Db::new(&args.db_path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            return;
        }
    };

    match args.table {
        Some(TableName::Words) => print_words(&db),
        Some(TableName::Meanings) => print_meanings(&db),
        Some(TableName::Closes) => print_clozes(&db),
        Some(TableName::Tags) => print_tags(&db),
        None => {
            print_tags(&db);
            println!();
            print_words(&db);
            println!();
            print_meanings(&db);
            println!();
            print_clozes(&db);
        }
    }
}
