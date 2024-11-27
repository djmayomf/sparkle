use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConcept {
    pub category: NetworkCategory,
    pub topic: String,
    pub summary: String,
    pub related_concepts: Vec<String>,
    pub key_terms: Vec<String>,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum NetworkCategory {
    NetworkArchitecture,
    Protocols,
    Infrastructure,
    Security,
    Troubleshooting,
    CloudNetworking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

pub struct NetworkKnowledge {
    concepts: HashMap<NetworkCategory, Vec<NetworkConcept>>,
    last_updated: chrono::DateTime<chrono::Utc>,
}

impl NetworkKnowledge {
    pub fn new() -> Self {
        Self {
            concepts: Self::init_concepts(),
            last_updated: chrono::Utc::now(),
        }
    }

    fn init_concepts() -> HashMap<NetworkCategory, Vec<NetworkConcept>> {
        let mut concepts = HashMap::new();
        
        // Network Architecture concepts
        let architecture_concepts = vec![
            NetworkConcept {
                category: NetworkCategory::NetworkArchitecture,
                topic: "OSI Model".to_string(),
                summary: "Framework that standardizes network communication into 7 layers".to_string(),
                related_concepts: vec!["TCP/IP Model".to_string(), "Encapsulation".to_string()],
                key_terms: vec!["Layer", "Protocol", "PDU".to_string()],
                difficulty: Difficulty::Beginner,
            },
            // Add more concepts...
        ];
        concepts.insert(NetworkCategory::NetworkArchitecture, architecture_concepts);

        // Add other categories...
        concepts
    }

    pub fn add_concept(&mut self, concept: NetworkConcept) {
        if let Some(category_concepts) = self.concepts.get_mut(&concept.category) {
            category_concepts.push(concept);
        } else {
            self.concepts.insert(concept.category.clone(), vec![concept]);
        }
        self.last_updated = chrono::Utc::now();
    }

    pub fn get_concepts_by_category(&self, category: &NetworkCategory) -> Option<&Vec<NetworkConcept>> {
        self.concepts.get(category)
    }

    pub fn search_concepts(&self, query: &str) -> Vec<&NetworkConcept> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for concepts in self.concepts.values() {
            for concept in concepts {
                if concept.topic.to_lowercase().contains(&query) 
                   || concept.summary.to_lowercase().contains(&query) {
                    results.push(concept);
                }
            }
        }
        results
    }
} 