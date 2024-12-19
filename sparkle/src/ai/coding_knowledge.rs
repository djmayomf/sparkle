use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CodingKnowledge {
    pub languages: HashMap<String, LanguageKnowledge>,
    pub concepts: HashMap<String, ProgrammingConcept>,
    pub tutorials: Vec<CodingTutorial>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageKnowledge {
    pub name: String,
    pub syntax: Vec<SyntaxRule>,
    pub best_practices: Vec<String>,
    pub common_patterns: Vec<CodePattern>,
    pub difficulty_level: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgrammingConcept {
    pub name: String,
    pub description: String,
    pub examples: HashMap<String, String>, // Language -> Example
    pub prerequisites: Vec<String>,
    pub difficulty: u8,
}

impl CodingKnowledge {
    pub async fn new() -> Self {
        let mut knowledge = Self {
            languages: HashMap::new(),
            concepts: HashMap::new(),
            tutorials: Vec::new(),
        };

        // Initialize with basic programming knowledge
        knowledge.initialize_languages().await;
        knowledge.initialize_concepts().await;
        knowledge.initialize_tutorials().await;

        knowledge
    }

    async fn initialize_languages(&mut self) {
        // Add Python knowledge
        self.languages.insert("python".to_string(), LanguageKnowledge {
            name: "Python".to_string(),
            syntax: vec![
                SyntaxRule {
                    name: "Variable Declaration".to_string(),
                    pattern: "variable_name = value".to_string(),
                    examples: vec!["x = 42".to_string(), "name = 'Alice'".to_string()],
                },
                // Add more syntax rules
            ],
            best_practices: vec![
                "Use snake_case for variables and functions".to_string(),
                "Follow PEP 8 style guide".to_string(),
            ],
            common_patterns: vec![
                CodePattern {
                    name: "List Comprehension".to_string(),
                    example: "[x for x in range(10)]".to_string(),
                    use_case: "Creating lists efficiently".to_string(),
                },
            ],
            difficulty_level: 1,
        });

        // Add Rust knowledge
        self.languages.insert("rust".to_string(), LanguageKnowledge {
            name: "Rust".to_string(),
            syntax: vec![
                SyntaxRule {
                    name: "Variable Declaration".to_string(),
                    pattern: "let variable_name: type = value;".to_string(),
                    examples: vec!["let x: i32 = 42;".to_string()],
                },
            ],
            best_practices: vec![
                "Follow ownership rules".to_string(),
                "Use Result for error handling".to_string(),
            ],
            common_patterns: vec![
                CodePattern {
                    name: "Result handling".to_string(),
                    example: "result?.do_something()".to_string(),
                    use_case: "Error propagation".to_string(),
                },
            ],
            difficulty_level: 3,
        });

        // Add more languages...
    }
} 