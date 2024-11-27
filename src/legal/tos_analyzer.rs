use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct TOSAnalyzer {
    pub platforms: HashMap<Platform, PlatformTOS>,
    pub compliance_checker: ComplianceChecker,
    pub last_updated: DateTime<Utc>,
    pub active_agreements: Vec<Agreement>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum Platform {
    Blizzard,
    RiotGames,
    Valorant,
    ApexLegends,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformTOS {
    pub platform: Platform,
    pub last_updated: DateTime<Utc>,
    pub key_points: Vec<KeyPoint>,
    pub restrictions: Vec<Restriction>,
    pub content_rules: Vec<ContentRule>,
    pub streaming_guidelines: StreamingGuidelines,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPoint {
    pub topic: String,
    pub description: String,
    pub importance: Importance,
    pub compliance_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamingGuidelines {
    pub allowed_content: Vec<String>,
    pub prohibited_content: Vec<String>,
    pub monetization_rules: Vec<String>,
    pub branding_requirements: Vec<String>,
}

impl TOSAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            platforms: HashMap::new(),
            compliance_checker: ComplianceChecker::new(),
            last_updated: Utc::now(),
            active_agreements: Vec::new(),
        };

        analyzer.initialize_platforms();
        analyzer
    }

    fn initialize_platforms(&mut self) {
        // Riot Games/Valorant TOS
        self.platforms.insert(
            Platform::RiotGames,
            PlatformTOS {
                platform: Platform::RiotGames,
                last_updated: Utc::now(),
                key_points: vec![
                    KeyPoint {
                        topic: "Voice Chat Recording".to_string(),
                        description: "Voice communications may be recorded and evaluated when reports are submitted".to_string(),
                        importance: Importance::Critical,
                        compliance_required: true,
                    },
                    KeyPoint {
                        topic: "Account Requirements".to_string(),
                        description: "Must be an adult or have parent/guardian consent".to_string(),
                        importance: Importance::Critical,
                        compliance_required: true,
                    },
                ],
                restrictions: vec![
                    Restriction {
                        name: "Cheating".to_string(),
                        description: "No use of unauthorized third-party programs or cheats".to_string(),
                        penalty: "Account termination".to_string(),
                    },
                    Restriction {
                        name: "Commercial Use".to_string(),
                        description: "No unauthorized commercial exploitation".to_string(),
                        penalty: "Account suspension/termination".to_string(),
                    },
                ],
                content_rules: vec![
                    ContentRule {
                        category: "Streaming".to_string(),
                        allowed: vec!["Personal streams".to_string()],
                        prohibited: vec!["Unauthorized commercial broadcasts".to_string()],
                    }
                ],
                streaming_guidelines: StreamingGuidelines {
                    allowed_content: vec![
                        "Personal streaming".to_string(),
                        "Community content".to_string(),
                    ],
                    prohibited_content: vec![
                        "Cheating/hacking content".to_string(),
                        "Unauthorized commercial use".to_string(),
                    ],
                    monetization_rules: vec![
                        "Standard platform monetization allowed".to_string(),
                    ],
                    branding_requirements: vec![
                        "Must maintain game integrity".to_string(),
                    ],
                },
            }
        );

        // Blizzard TOS
        self.platforms.insert(
            Platform::Blizzard,
            PlatformTOS {
                platform: Platform::Blizzard,
                last_updated: Utc::now(),
                key_points: vec![
                    KeyPoint {
                        topic: "License".to_string(),
                        description: "Limited, revocable, non-sub licensable license".to_string(),
                        importance: Importance::Critical,
                        compliance_required: true,
                    },
                ],
                restrictions: vec![
                    Restriction {
                        name: "Cheating".to_string(),
                        description: "No cheats, bots, or unauthorized modifications".to_string(),
                        penalty: "Account termination".to_string(),
                    },
                ],
                content_rules: vec![],
                streaming_guidelines: StreamingGuidelines::default(),
            }
        );
    }

    pub async fn check_compliance(&self, content_type: &str) -> Result<ComplianceReport, String> {
        let mut report = ComplianceReport {
            timestamp: Utc::now(),
            content_type: content_type.to_string(),
            violations: Vec::new(),
            recommendations: Vec::new(),
            is_compliant: true,
        };

        // Check against each platform's rules
        for (platform, tos) in &self.platforms {
            let platform_compliance = self.compliance_checker.check_platform_rules(
                platform,
                content_type,
                &tos.restrictions,
                &tos.content_rules
            )?;

            if !platform_compliance.is_compliant {
                report.is_compliant = false;
                report.violations.extend(platform_compliance.violations);
                report.recommendations.extend(platform_compliance.recommendations);
            }
        }

        Ok(report)
    }

    pub async fn get_streaming_guidelines(&self) -> Vec<String> {
        let mut guidelines = Vec::new();
        
        for (_, tos) in &self.platforms {
            guidelines.extend(tos.streaming_guidelines.allowed_content.clone());
            guidelines.extend(tos.streaming_guidelines.prohibited_content.clone());
            guidelines.extend(tos.streaming_guidelines.monetization_rules.clone());
        }

        guidelines
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceChecker {
    pub rules: Vec<ComplianceRule>,
    pub violation_history: Vec<Violation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceRule {
    pub name: String,
    pub description: String,
    pub platforms: Vec<Platform>,
    pub check_function: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplianceReport {
    pub timestamp: DateTime<Utc>,
    pub content_type: String,
    pub violations: Vec<Violation>,
    pub recommendations: Vec<String>,
    pub is_compliant: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Violation {
    pub platform: Platform,
    pub rule: String,
    pub description: String,
    pub severity: Severity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Importance {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Restriction {
    pub name: String,
    pub description: String,
    pub penalty: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContentRule {
    pub category: String,
    pub allowed: Vec<String>,
    pub prohibited: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Agreement {
    pub platform: Platform,
    pub agreed_date: DateTime<Utc>,
    pub version: String,
    pub key_terms: Vec<String>,
} 