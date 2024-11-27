use crate::error::Result;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use crate::constants::strings::{SECURITY_MESSAGES, SECURITY_RECOMMENDATIONS, SecurityPaths};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    timestamp: u64,
    event_type: String,
    source: String,
    description: String,
    severity: String,
    recommendations: Vec<String>,
}

pub struct SecurityScraper {
    client: Client,
    cache: HashMap<String, (SecurityEvent, SystemTime)>,
}

impl SecurityScraper {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(crate::constants::strings::SECURITY_SCANNER_UA)
            .build()?;

        Ok(Self {
            client,
            cache: HashMap::new(),
        })
    }

    pub async fn get_security_insights(&mut self) -> Result<Vec<SecurityEvent>> {
        // Clean old cache entries
        self.cleanup_cache();

        // Key security concepts from Splunk for monitoring
        let security_patterns = vec![
            // Search patterns
            "index=security sourcetype=*security*", 
            "index=network sourcetype=firewall_*",
            "index=endpoint sourcetype=winlog:security",
            
            // Common security events
            "failed login attempt",
            "successful privilege elevation",
            "firewall block",
            "malware detected",
            "unusual network traffic",
            
            // Critical alerts
            "severity=critical category=security",
            "alert_severity=high sourcetype=IDS",
            "priority=1 source=firewall"
        ];

        let mut events = Vec::new();

        for pattern in security_patterns {
            if let Some((event, _)) = self.cache.get(pattern) {
                events.push(event.clone());
                continue;
            }

            let event = SecurityEvent {
                timestamp: SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs(),
                event_type: "Security Alert".to_string(),
                source: pattern.to_string(),
                description: self.analyze_pattern(pattern).await?,
                severity: self.determine_severity(pattern),
                recommendations: self.get_recommendations(pattern),
            };

            self.cache.insert(
                pattern.to_string(),
                (event.clone(), SystemTime::now())
            );
            events.push(event);
        }

        Ok(events)
    }

    async fn analyze_pattern(&self, pattern: &str) -> Result<String> {
        let analysis = match pattern {
            p if p.contains("failed login") => 
                SECURITY_MESSAGES.get("failed_login"),
            p if p.contains("privilege elevation") => 
                SECURITY_MESSAGES.get("privilege_elevation"),
            p if p.contains("firewall") => 
                SECURITY_MESSAGES.get("firewall_event"),
            p if p.contains("malware") => 
                SECURITY_MESSAGES.get("malware_detected"),
            p if p.contains("unusual") => 
                SECURITY_MESSAGES.get("unusual_traffic"),
            _ => SECURITY_MESSAGES.get("generic_event"),
        };

        Ok(analysis.unwrap_or(&"").to_string())
    }

    fn determine_severity(&self, pattern: &str) -> String {
        // Determine severity based on Splunk severity levels
        if pattern.contains("critical") || pattern.contains("priority=1") {
            "Critical".to_string()
        } else if pattern.contains("high") {
            "High".to_string()
        } else if pattern.contains("failed") || pattern.contains("unusual") {
            "Medium".to_string()
        } else {
            "Low".to_string()
        }
    }

    fn get_recommendations(&self, pattern: &str) -> Vec<String> {
        let mut recommendations = Vec::new();

        for (key, recs) in SECURITY_RECOMMENDATIONS.iter() {
            if pattern.contains(key) {
                recommendations.extend(recs.iter().map(|&s| s.to_string()));
            }
        }

        recommendations
    }

    fn cleanup_cache(&mut self) {
        const MAX_AGE: Duration = Duration::from_secs(3600); // 1 hour
        self.cache.retain(|_, (_, timestamp)| {
            timestamp.elapsed().unwrap_or(MAX_AGE) < MAX_AGE
        });
    }

    pub async fn monitor_security_events(&mut self) -> Result<()> {
        // Continuous security monitoring
        loop {
            let events = self.get_security_insights().await?;
            
            for event in events {
                if event.severity == "Critical" || event.severity == "High" {
                    println!("⚠️ High Priority Security Event:");
                    println!("Type: {}", event.event_type);
                    println!("Source: {}", event.source);
                    println!("Description: {}", event.description);
                    println!("Severity: {}", event.severity);
                    println!("Recommendations:");
                    for rec in event.recommendations {
                        println!("- {}", rec);
                    }
                    println!();
                }
            }

            tokio::time::sleep(Duration::from_secs(300)).await; // Check every 5 minutes
        }
    }
} 