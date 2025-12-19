use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_PATTERNS_FILE: &str = "regex.json";

#[derive(Debug, Clone)]
pub struct SecretPattern {
    pub name: String,
    pub pattern: Regex,
    pub description: String,
}

#[derive(Debug, Deserialize)]
struct PatternEntry {
    name: String,
    regex: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct PatternsFile {
    patterns: Vec<PatternEntry>,
}


/// Charger les regexe depuis le fichier json
pub fn load_patterns_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<SecretPattern>, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read patterns file: {}", e))?;
    
    let patterns_file: PatternsFile = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse patterns JSON: {}", e))?;
    
    let mut patterns = Vec::new();
    for entry in patterns_file.patterns {
        match Regex::new(&entry.regex) {
            Ok(regex) => patterns.push(SecretPattern {
                name: entry.name,
                pattern: regex,
                description: entry.description,
            }),
            Err(e) => {
                eprintln!("Warning: Invalid regex for pattern '{}': {}", entry.name, e);
            }
        }
    }
    
    Ok(patterns)
}

pub fn get_default_patterns() -> Vec<SecretPattern> {
    match load_patterns_from_file(DEFAULT_PATTERNS_FILE) {
        Ok(patterns) => patterns,
        Err(e) => {
            eprintln!("Error loading patterns: {}", e);
            Vec::new()
        }
    }
}
