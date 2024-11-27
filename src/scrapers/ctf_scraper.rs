use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Debug, Serialize, Deserialize)]
pub struct CTFKnowledgeBase {
    pub categories: HashMap<String, CTFCategory>,
    pub tools: HashMap<String, CTFTool>,
    pub techniques: HashMap<String, CTFTechnique>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTFCategory {
    pub name: String,
    pub overview: String,
    pub subcategories: Vec<String>,
    pub common_tools: Vec<String>,
    pub learning_resources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTFTool {
    pub name: String,
    pub purpose: String,
    pub category: String,
    pub usage_examples: Vec<String>,
    pub installation: String,
    pub common_flags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CTFTechnique {
    pub name: String,
    pub description: String,
    pub difficulty: String,
    pub prerequisites: Vec<String>,
    pub steps: Vec<String>,
    pub common_pitfalls: Vec<String>,
}

pub struct CTFScraper {
    client: Client,
    base_url: String,
    knowledge_base: CTFKnowledgeBase,
}

impl CTFScraper {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: Client::new(),
            base_url: "https://ctf101.org".to_string(),
            knowledge_base: CTFKnowledgeBase {
                categories: Self::initialize_categories(),
                tools: HashMap::new(),
                techniques: HashMap::new(),
                last_updated: chrono::Utc::now(),
            },
        })
    }

    fn initialize_categories() -> HashMap<String, CTFCategory> {
        let mut categories = HashMap::new();

        // Based on CTF101.org structure
        categories.insert(
            "forensics".to_string(),
            CTFCategory {
                name: "Forensics".to_string(),
                overview: "Digital forensics in CTF competitions".to_string(),
                subcategories: vec![
                    "File Formats".to_string(),
                    "Metadata".to_string(),
                    "Steganography".to_string(),
                    "Disk Imaging".to_string(),
                    "Memory Forensics".to_string(),
                    "Network Analysis".to_string(),
                ],
                common_tools: vec![
                    "Wireshark".to_string(),
                    "Volatility".to_string(),
                    "Autopsy".to_string(),
                ],
                learning_resources: vec![],
            }
        );

        categories.insert(
            "cryptography".to_string(),
            CTFCategory {
                name: "Cryptography".to_string(),
                overview: "Cryptographic challenges and techniques".to_string(),
                subcategories: vec![
                    "XOR".to_string(),
                    "Hashing Functions".to_string(),
                    "Substitution Cipher".to_string(),
                    "Caesar Cipher".to_string(),
                    "Vigenere Cipher".to_string(),
                    "RSA".to_string(),
                ],
                common_tools: vec![
                    "CyberChef".to_string(),
                    "OpenSSL".to_string(),
                    "RsaCtfTool".to_string(),
                ],
                learning_resources: vec![],
            }
        );

        categories.insert(
            "web_exploitation".to_string(),
            CTFCategory {
                name: "Web Exploitation".to_string(),
                overview: "Web security challenges".to_string(),
                subcategories: vec![
                    "SQL Injection".to_string(),
                    "Command Injection".to_string(),
                    "Directory Traversal".to_string(),
                    "Cross Site Request Forgery".to_string(),
                    "Cross Site Scripting".to_string(),
                    "Server Side Request Forgery".to_string(),
                ],
                common_tools: vec![
                    "Burp Suite".to_string(),
                    "OWASP ZAP".to_string(),
                    "SQLMap".to_string(),
                ],
                learning_resources: vec![],
            }
        );

        categories
    }

    pub async fn update_knowledge_base(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Scrape main categories
        self.scrape_forensics().await?;
        self.scrape_cryptography().await?;
        self.scrape_web_exploitation().await?;
        self.scrape_reverse_engineering().await?;
        self.scrape_binary_exploitation().await?;
        
        self.knowledge_base.last_updated = chrono::Utc::now();
        Ok(())
    }

    async fn scrape_forensics(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/forensics/overview", self.base_url);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        // Parse forensics content
        // Update knowledge base
        
        Ok(())
    }

    async fn scrape_cryptography(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/cryptography/overview", self.base_url);
        let response = self.client.get(&url).send().await?;
        let html = response.text().await?;
        let document = Html::parse_document(&html);

        // Parse cryptography content
        // Update knowledge base
        
        Ok(())
    }

    pub async fn get_challenge_hints(&self, category: &str, challenge_type: &str) -> Option<Vec<String>> {
        if let Some(cat) = self.knowledge_base.categories.get(category) {
            // Return relevant hints based on category and challenge type
            Some(vec![
                format!("Check common tools for {}: {:?}", cat.name, cat.common_tools),
                format!("Review techniques for: {:?}", cat.subcategories),
            ])
        } else {
            None
        }
    }

    pub async fn get_tool_usage(&self, tool_name: &str) -> Option<CTFTool> {
        self.knowledge_base.tools.get(tool_name).cloned()
    }
} 