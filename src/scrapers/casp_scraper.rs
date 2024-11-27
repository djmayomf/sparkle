use pdf::file::File as PdfFile;
use pdf::content::Content;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct CASPConcept {
    pub domain: String,
    pub topic: String,
    pub description: String,
    pub key_points: Vec<String>,
    pub related_concepts: Vec<String>,
    pub difficulty_level: String,
}

pub struct CASPScraper {
    pdf_path: String,
    cache: HashMap<String, CASPConcept>,
}

impl CASPScraper {
    pub fn new(pdf_path: String) -> Self {
        Self {
            pdf_path,
            cache: HashMap::new(),
        }
    }

    pub async fn extract_concepts(&mut self) -> Result<Vec<CASPConcept>, Box<dyn std::error::Error>> {
        let file = PdfFile::open(&self.pdf_path)?;
        let mut concepts = Vec::new();

        for page in file.pages() {
            let content = page.content()?;
            let text = content.text();
            
            if let Some(concept) = self.parse_domain_content(&text) {
                concepts.push(concept);
            }
        }

        Ok(concepts)
    }

    pub async fn update_knowledge_base(&mut self, knowledge_base: &mut crate::knowledge::base::KnowledgeBase) -> Result<(), Box<dyn std::error::Error>> {
        let concepts = self.extract_concepts().await?;

        for concept in concepts {
            let info = crate::knowledge::base::SecurityInfo {
                topic: concept.topic,
                description: concept.description,
                difficulty: concept.difficulty_level,
                real_world_examples: Vec::new(), // Will be populated from other sources
                best_practices: concept.key_points,
                tools: Vec::new(),
                resources: vec!["CASP+ Study Materials".to_string()],
            };

            knowledge_base.add_security_info(&concept.domain, info);
        }

        Ok(())
    }

    fn parse_domain_content(&self, text: &str) -> Option<CASPConcept> {
        // Extract domain information using regex patterns
        let domain_pattern = regex::Regex::new(r"Domain \d+:?\s*(.+)").unwrap();
        let topic_pattern = regex::Regex::new(r"(?m)^[A-Z][^.]+$").unwrap();
        
        if let Some(captures) = domain_pattern.captures(text) {
            let domain = captures[1].to_string();
            let topic = topic_pattern.find(text)?.as_str().to_string();
            
            Some(CASPConcept {
                domain,
                topic: topic.clone(),
                description: self.extract_description(text),
                key_points: self.extract_key_points(text),
                related_concepts: self.extract_related_concepts(text),
                difficulty_level: self.determine_difficulty(&topic),
            })
        } else {
            None
        }
    }

    fn determine_difficulty(&self, topic: &str) -> String {
        // Determine difficulty based on topic complexity
        if topic.contains("advanced") || topic.contains("enterprise") {
            "Advanced".to_string()
        } else if topic.contains("implementation") || topic.contains("analysis") {
            "Intermediate".to_string()
        } else {
            "Foundational".to_string()
        }
    }
} 