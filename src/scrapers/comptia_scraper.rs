use pdf::file::File as PdfFile;
use pdf::content::Content;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityConcept {
    pub topic: String,
    pub description: String,
    pub key_points: Vec<String>,
    pub examples: Vec<String>,
    pub related_concepts: Vec<String>,
    pub chapter: String,
    pub difficulty: String,
}

pub struct CompTIAScraper {
    pdf_path: String,
    cache: HashMap<String, SecurityConcept>,
}

impl CompTIAScraper {
    pub fn new(pdf_path: String) -> Self {
        Self {
            pdf_path,
            cache: HashMap::new(),
        }
    }

    pub async fn extract_security_concepts(&mut self) -> Result<Vec<SecurityConcept>, Box<dyn std::error::Error>> {
        let file = PdfFile::open(&self.pdf_path)?;
        let mut concepts = Vec::new();

        for page in file.pages() {
            let content = page.content()?;
            let text = content.text();
            
            // Extract chapter headings and content
            if let Some(concept) = self.parse_chapter_content(&text) {
                concepts.push(concept);
            }
        }

        Ok(concepts)
    }

    pub async fn update_security_knowledge(&mut self, knowledge_base: &mut crate::knowledge::base::KnowledgeBase) -> Result<(), Box<dyn std::error::Error>> {
        let concepts = self.extract_security_concepts().await?;

        for concept in concepts {
            let info = crate::knowledge::base::SecurityInfo {
                topic: concept.topic,
                description: concept.description,
                difficulty: concept.difficulty,
                real_world_examples: concept.examples,
                best_practices: concept.key_points,
                tools: Vec::new(), // Tools are typically mentioned within the description
                resources: vec![format!("CompTIA Security+ Study Guide Chapter: {}", concept.chapter)],
            };

            knowledge_base.add_security_info(&concept.topic, info);
        }

        Ok(())
    }

    fn parse_chapter_content(&self, text: &str) -> Option<SecurityConcept> {
        // Extract chapter information using regex patterns
        let chapter_pattern = regex::Regex::new(r"Chapter \d+:?\s*(.+)").unwrap();
        let topic_pattern = regex::Regex::new(r"(?m)^[A-Z][^.]+$").unwrap();
        
        if let Some(captures) = chapter_pattern.captures(text) {
            let chapter = captures[1].to_string();
            let topic = topic_pattern.find(text)?.as_str().to_string();
            
            Some(SecurityConcept {
                topic: topic.clone(),
                description: self.extract_description(text),
                key_points: self.extract_key_points(text),
                examples: self.extract_examples(text),
                related_concepts: self.extract_related_concepts(text),
                chapter,
                difficulty: self.determine_difficulty(&topic, text),
            })
        } else {
            None
        }
    }
} 