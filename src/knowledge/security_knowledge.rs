use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityKnowledge {
    topics: HashMap<SecurityTopic, Vec<KnowledgeEntry>>,
    references: Vec<Reference>,
    last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum SecurityTopic {
    NetworkSecurity,
    WebSecurity,
    SystemSecurity,
    EthicalHacking,
    Malware,
    Cryptography,
    IncidentResponse,
    ComplianceAndRegulations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEntry {
    pub title: String,
    pub description: String,
    pub key_points: Vec<String>,
    pub examples: Vec<String>,
    pub difficulty: Difficulty,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl SecurityKnowledge {
    pub fn new() -> Self {
        let mut topics = HashMap::new();
        
        // Network Security
        topics.insert(SecurityTopic::NetworkSecurity, vec![
            KnowledgeEntry {
                title: "Network Protocols and Security".to_string(),
                description: "Understanding common network protocols and their security implications".to_string(),
                key_points: vec![
                    "TCP/IP fundamentals".to_string(),
                    "Common network attacks".to_string(),
                    "Network defense strategies".to_string(),
                ],
                examples: vec![
                    "TCP handshake process".to_string(),
                    "DDoS attack mitigation".to_string(),
                ],
                difficulty: Difficulty::Intermediate,
                tags: vec!["networking".to_string(), "protocols".to_string()],
            }
        ]);

        // Web Security
        topics.insert(SecurityTopic::WebSecurity, vec![
            KnowledgeEntry {
                title: "Common Web Vulnerabilities".to_string(),
                description: "Understanding and preventing common web application vulnerabilities".to_string(),
                key_points: vec![
                    "SQL Injection".to_string(),
                    "Cross-Site Scripting (XSS)".to_string(),
                    "CSRF attacks".to_string(),
                ],
                examples: vec![
                    "Input validation techniques".to_string(),
                    "Secure cookie handling".to_string(),
                ],
                difficulty: Difficulty::Intermediate,
                tags: vec!["web".to_string(), "vulnerabilities".to_string()],
            }
        ]);

        // System Security
        topics.insert(SecurityTopic::SystemSecurity, vec![
            KnowledgeEntry {
                title: "Operating System Security".to_string(),
                description: "Fundamental concepts of OS security and hardening".to_string(),
                key_points: vec![
                    "Access control mechanisms".to_string(),
                    "File system security".to_string(),
                    "Process isolation".to_string(),
                ],
                examples: vec![
                    "Linux file permissions".to_string(),
                    "Windows security policies".to_string(),
                ],
                difficulty: Difficulty::Intermediate,
                tags: vec!["os".to_string(), "hardening".to_string()],
            }
        ]);

        SecurityKnowledge {
            topics,
            references: vec![],
            last_updated: chrono::Utc::now(),
        }
    }

    pub fn get_topic_info(&self, topic: &SecurityTopic) -> Option<&Vec<KnowledgeEntry>> {
        self.topics.get(topic)
    }

    pub fn add_knowledge_entry(&mut self, topic: SecurityTopic, entry: KnowledgeEntry) {
        if let Some(entries) = self.topics.get_mut(&topic) {
            entries.push(entry);
        } else {
            self.topics.insert(topic, vec![entry]);
        }
    }

    pub fn search_by_tag(&self, tag: &str) -> Vec<&KnowledgeEntry> {
        let mut results = Vec::new();
        for entries in self.topics.values() {
            for entry in entries {
                if entry.tags.iter().any(|t| t.to_lowercase() == tag.to_lowercase()) {
                    results.push(entry);
                }
            }
        }
        results
    }

    pub fn get_by_difficulty(&self, difficulty: Difficulty) -> Vec<&KnowledgeEntry> {
        let mut results = Vec::new();
        for entries in self.topics.values() {
            for entry in entries {
                if std::mem::discriminant(&entry.difficulty) == std::mem::discriminant(&difficulty) {
                    results.push(entry);
                }
            }
        }
        results
    }

    pub fn update_entry(&mut self, topic: &SecurityTopic, title: &str, updated_entry: KnowledgeEntry) -> bool {
        if let Some(entries) = self.topics.get_mut(topic) {
            if let Some(index) = entries.iter().position(|e| e.title == title) {
                entries[index] = updated_entry;
                return true;
            }
        }
        false
    }
} 