use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityTopic {
    pub name: String,
    pub description: String,
    pub difficulty: SecurityLevel,
    pub prerequisites: Vec<String>,
    pub resources: Vec<String>,
    pub real_world_examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VulnerabilityExample {
    pub cve_id: Option<String>,
    pub name: String,
    pub description: String,
    pub impact: String,
    pub mitigation: String,
    pub references: Vec<String>,
}

pub struct SecurityKnowledgeBase {
    topics: HashMap<String, SecurityTopic>,
    vulnerabilities: Vec<VulnerabilityExample>,
    learning_paths: HashMap<String, Vec<String>>, // path_name -> ordered_topics
}

impl SecurityKnowledgeBase {
    pub fn new() -> Self {
        Self {
            topics: Self::init_topics(),
            vulnerabilities: Self::init_vulnerabilities(),
            learning_paths: Self::init_learning_paths(),
        }
    }

    fn init_topics() -> HashMap<String, SecurityTopic> {
        let mut topics = HashMap::new();
        
        // Basic Network Security
        topics.insert("network_basics".to_string(), SecurityTopic {
            name: "Network Security Fundamentals".to_string(),
            description: "Understanding basic network security concepts, protocols, and common threats".to_string(),
            difficulty: SecurityLevel::Beginner,
            prerequisites: vec![],
            resources: vec![
                "https://www.cybrary.it/course/network-security-fundamentals".to_string(),
                "https://www.sans.org/security-awareness-training".to_string(),
            ],
            real_world_examples: vec![
                "Recent DDoS attacks on major gaming services".to_string(),
                "Man-in-the-middle attacks in public WiFi networks".to_string(),
            ],
        });

        // Web Security
        topics.insert("web_security".to_string(), SecurityTopic {
            name: "Web Application Security".to_string(),
            description: "Understanding common web vulnerabilities, OWASP Top 10, and secure coding practices".to_string(),
            difficulty: SecurityLevel::Intermediate,
            prerequisites: vec!["network_basics".to_string()],
            resources: vec![
                "https://owasp.org/www-project-top-ten".to_string(),
                "https://portswigger.net/web-security".to_string(),
            ],
            real_world_examples: vec![
                "SQL injection attacks on major databases".to_string(),
                "Cross-site scripting in social media platforms".to_string(),
            ],
        });

        // Add more topics...
        topics
    }

    fn init_vulnerabilities() -> Vec<VulnerabilityExample> {
        vec![
            VulnerabilityExample {
                cve_id: Some("CVE-2021-44228".to_string()),
                name: "Log4Shell".to_string(),
                description: "Critical vulnerability in Log4j allowing remote code execution".to_string(),
                impact: "Allows attackers to execute arbitrary code on affected systems".to_string(),
                mitigation: "Update Log4j to version 2.15.0 or higher".to_string(),
                references: vec![
                    "https://nvd.nist.gov/vuln/detail/CVE-2021-44228".to_string(),
                ],
            },
            // Add more vulnerabilities...
        ]
    }

    fn init_learning_paths() -> HashMap<String, Vec<String>> {
        let mut paths = HashMap::new();
        
        paths.insert(
            "pentester".to_string(),
            vec![
                "network_basics".to_string(),
                "web_security".to_string(),
                "exploitation".to_string(),
            ],
        );

        paths.insert(
            "defender".to_string(),
            vec![
                "network_basics".to_string(),
                "incident_response".to_string(),
                "threat_hunting".to_string(),
            ],
        );

        paths
    }

    pub fn get_topic_info(&self, topic: &str) -> Option<&SecurityTopic> {
        self.topics.get(topic)
    }

    pub fn suggest_learning_path(&self, interest: &str) -> Option<Vec<&SecurityTopic>> {
        self.learning_paths.get(interest).map(|topics| {
            topics.iter()
                .filter_map(|topic| self.topics.get(topic))
                .collect()
        })
    }

    pub fn get_recent_vulnerabilities(&self, count: usize) -> Vec<&VulnerabilityExample> {
        self.vulnerabilities.iter().take(count).collect()
    }
} 