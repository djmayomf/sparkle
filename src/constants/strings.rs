//! String constants library for Kamen-Sparkle
use lazy_static::lazy_static;
use std::collections::HashMap;

// User Agent Strings
pub const USER_AGENT: &str = "KamenSparkle/1.0";
pub const SECURITY_SCANNER_UA: &str = "KamenSparkle Security Scanner/1.0";

// API Endpoints and URLs
pub struct Endpoints {
    pub base_urls: BaseUrls,
    pub api_paths: ApiPaths,
}

pub struct BaseUrls {
    pub anime_news: &'static str,
    pub security_news: &'static str,
    pub tech_news: &'static str,
    pub dark_reading: &'static str,
}

pub struct ApiPaths {
    pub security: SecurityPaths,
    pub anime: AnimePaths,
    pub tech: TechPaths,
}

impl Endpoints {
    pub fn new() -> Self {
        Self {
            base_urls: BaseUrls {
                anime_news: "https://www.animenewsnetwork.com",
                security_news: "https://www.darkreading.com",
                tech_news: "https://www.technewsworld.com",
                dark_reading: "https://www.darkreading.com",
            },
            api_paths: ApiPaths {
                security: SecurityPaths::new(),
                anime: AnimePaths::new(),
                tech: TechPaths::new(),
            },
        }
    }
}

pub struct SecurityPaths {
    pub categories: Vec<&'static str>,
    pub search_patterns: Vec<&'static str>,
}

impl SecurityPaths {
    pub fn new() -> Self {
        Self {
            categories: vec![
                "application-security",
                "cloud-security",
                "endpoint-security",
                "threat-intelligence",
                "vulnerabilities-threats",
                "ics-ot-security",
            ],
            search_patterns: vec![
                "index=security sourcetype=*security*",
                "index=network sourcetype=firewall_*",
                "index=endpoint sourcetype=winlog:security",
            ],
        }
    }
}

// Security Event Messages
lazy_static! {
    pub static ref SECURITY_MESSAGES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("failed_login", "Potential brute force attack detected. Monitor authentication attempts.");
        m.insert("privilege_elevation", "Privilege escalation event - verify if authorized.");
        m.insert("firewall_event", "Network security event - analyze traffic patterns.");
        m.insert("malware_detected", "Malware activity detected - initiate incident response.");
        m.insert("unusual_traffic", "Anomalous behavior detected - investigate potential threats.");
        m.insert("generic_event", "Generic security event - requires investigation.");
        m
    };
}

// Security Recommendations
lazy_static! {
    pub static ref SECURITY_RECOMMENDATIONS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("login", vec![
            "Review authentication logs",
            "Implement multi-factor authentication",
            "Update password policies"
        ]);
        m.insert("privilege", vec![
            "Audit user permissions",
            "Review access control policies",
            "Enable detailed audit logging"
        ]);
        m.insert("firewall", vec![
            "Review firewall rules",
            "Analyze network traffic patterns",
            "Update security policies"
        ]);
        m.insert("malware", vec![
            "Isolate affected systems",
            "Update antivirus signatures",
            "Scan for indicators of compromise"
        ]);
        m
    };
}

// Severity Levels
pub const SEVERITY_CRITICAL: &str = "Critical";
pub const SEVERITY_HIGH: &str = "High";
pub const SEVERITY_MEDIUM: &str = "Medium";
pub const SEVERITY_LOW: &str = "Low";

// Chat Response Templates
pub struct ChatTemplates {
    pub greetings: Vec<&'static str>,
    pub farewells: Vec<&'static str>,
    pub reactions: Vec<&'static str>,
}

impl ChatTemplates {
    pub fn new() -> Self {
        Self {
            greetings: vec![
                "こんにちは~ (｡♥‿♥｡)",
                "Hello everyone! ✨",
                "Hi hi! Ready to hack some hearts? 💖",
            ],
            farewells: vec![
                "Bye bye! Stay secure! 💝",
                "またね～ (´｡• ᵕ •｡`) ♡",
                "See you next time! Remember to update your passwords! 🔐✨",
            ],
            reactions: vec![
                "Sugoi! (◕‿◕✿)",
                "Kawaii! ♪(๑ᴖ◡ᴖ๑)♪",
                "Yatta! ٩(◕‿◕｡)۶",
            ],
        }
    }
}

// Error Messages
pub struct ErrorMessages {
    pub database: &'static str,
    pub network: &'static str,
    pub scraping: &'static str,
    pub auth: &'static str,
}

impl ErrorMessages {
    pub fn new() -> Self {
        Self {
            database: "Database connection error: {}",
            network: "Network request failed: {}",
            scraping: "Failed to scrape content: {}",
            auth: "Authentication failed: {}",
        }
    }
}

// Environment Variables
pub struct EnvVars {
    pub required: Vec<&'static str>,
    pub optional: Vec<&'static str>,
}

impl EnvVars {
    pub fn new() -> Self {
        Self {
            required: vec![
                "DATABASE_URL",
                "YOUTUBE_API_KEY",
                "TWITCH_CLIENT_ID",
                "TWITCH_CLIENT_SECRET",
            ],
            optional: vec![
                "LOG_LEVEL",
                "CACHE_DURATION",
                "MAX_RETRIES",
            ],
        }
    }
} 