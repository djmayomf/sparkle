use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeBase {
    tokusatsu: HashMap<String, TokusatsuInfo>,
    anime: HashMap<String, AnimeInfo>,
    cybersecurity: HashMap<String, SecurityInfo>,
    gaming: HashMap<String, GamingInfo>,
    response_cache: HashMap<String, CachedResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokusatsuInfo {
    title: String,
    description: String,
    year: u32,
    notable_characters: Vec<String>,
    fun_facts: Vec<String>,
    related_series: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimeInfo {
    title: String,
    genre: Vec<String>,
    synopsis: String,
    recommendations: Vec<String>,
    fun_facts: Vec<String>,
    seasonal_info: Option<SeasonalAnime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityInfo {
    topic: String,
    description: String,
    difficulty: String,
    real_world_examples: Vec<String>,
    best_practices: Vec<String>,
    tools: Vec<String>,
    resources: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GamingInfo {
    game: String,
    genre: Vec<String>,
    tips: Vec<String>,
    meta_strategies: Vec<String>,
    patch_notes: Vec<PatchNote>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonalAnime {
    season: String,
    year: u32,
    popularity_rank: u32,
    currently_airing: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatchNote {
    version: String,
    date: DateTime<Utc>,
    changes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedResponse {
    content: String,
    timestamp: DateTime<Utc>,
    access_count: u32,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        Self {
            tokusatsu: Self::init_tokusatsu(),
            anime: Self::init_anime(),
            cybersecurity: Self::init_cybersecurity(),
            gaming: Self::init_gaming(),
            response_cache: HashMap::new(),
        }
    }

    fn init_tokusatsu() -> HashMap<String, TokusatsuInfo> {
        let mut map = HashMap::new();
        
        map.insert("kamen_rider".to_string(), TokusatsuInfo {
            title: "Kamen Rider".to_string(),
            description: "A Japanese tokusatsu series created by Shotaro Ishinomori in 1971".to_string(),
            year: 1971,
            notable_characters: vec![
                "Takeshi Hongo".to_string(),
                "Hayato Ichimonji".to_string(),
            ],
            fun_facts: vec![
                "The first suit weighed over 25kg!".to_string(),
                "The iconic Rider Kick was created because the suit was too stiff for complex fight scenes".to_string(),
            ],
            related_series: vec![
                "Kamen Rider V3".to_string(),
                "Kamen Rider Black".to_string(),
                "Kamen Rider Kuuga".to_string(),
            ],
        });

        map.insert("super_sentai".to_string(), TokusatsuInfo {
            title: "Super Sentai".to_string(),
            description: "The original Japanese series that inspired Power Rangers".to_string(),
            year: 1975,
            notable_characters: vec![
                "AkaRanger".to_string(),
                "Big One".to_string(),
            ],
            fun_facts: vec![
                "Himitsu Sentai Gorenger was the first Super Sentai series".to_string(),
                "The franchise has been running continuously for over 45 years!".to_string(),
            ],
            related_series: vec![
                "Power Rangers".to_string(),
                "Metal Heroes".to_string(),
            ],
        });

        // Add more tokusatsu entries...
        map
    }

    fn init_anime() -> HashMap<String, AnimeInfo> {
        let mut map = HashMap::new();
        
        map.insert("ghost_in_the_shell".to_string(), AnimeInfo {
            title: "Ghost in the Shell".to_string(),
            genre: vec!["Cyberpunk".to_string(), "Sci-fi".to_string(), "Action".to_string()],
            synopsis: "In a cyberized future, Major Motoko Kusanagi leads Public Security Section 9...".to_string(),
            recommendations: vec![
                "Serial Experiments Lain".to_string(),
                "Psycho-Pass".to_string(),
                "Akira".to_string(),
            ],
            fun_facts: vec![
                "Influenced The Matrix creators".to_string(),
                "Based on manga by Masamune Shirow".to_string(),
            ],
            seasonal_info: None,
        });

        // Add current seasonal anime
        map.insert("current_season".to_string(), AnimeInfo {
            title: "Current Season Highlights".to_string(),
            genre: vec!["Various".to_string()],
            synopsis: "Current season's most popular shows".to_string(),
            recommendations: vec![],
            fun_facts: vec![],
            seasonal_info: Some(SeasonalAnime {
                season: "Winter".to_string(),
                year: 2024,
                popularity_rank: 1,
                currently_airing: true,
            }),
        });

        // Add more anime entries...
        map
    }

    fn init_cybersecurity() -> HashMap<String, SecurityInfo> {
        let mut map = HashMap::new();
        
        map.insert("penetration_testing".to_string(), SecurityInfo {
            topic: "Penetration Testing".to_string(),
            description: "Authorized simulated cyberattack to evaluate system security".to_string(),
            difficulty: "Intermediate to Advanced".to_string(),
            real_world_examples: vec![
                "HackerOne bug bounty programs".to_string(),
                "Red Team assessments for Fortune 500 companies".to_string(),
            ],
            best_practices: vec![
                "Always get written permission".to_string(),
                "Document everything thoroughly".to_string(),
                "Follow responsible disclosure".to_string(),
            ],
            tools: vec![
                "Metasploit".to_string(),
                "Burp Suite".to_string(),
                "Nmap".to_string(),
            ],
            resources: vec![
                "OWASP Testing Guide".to_string(),
                "HackerOne CTF".to_string(),
                "TryHackMe".to_string(),
            ],
        });

        map.insert("social_engineering".to_string(), SecurityInfo {
            topic: "Social Engineering".to_string(),
            description: "Psychological manipulation for information gathering".to_string(),
            difficulty: "Beginner to Intermediate".to_string(),
            real_world_examples: vec![
                "Twitter crypto account takeovers".to_string(),
                "Tech support scams".to_string(),
            ],
            best_practices: vec![
                "Verify caller identity".to_string(),
                "Use multi-factor authentication".to_string(),
                "Train employees regularly".to_string(),
            ],
            tools: vec![
                "SET (Social Engineering Toolkit)".to_string(),
                "Maltego".to_string(),
            ],
            resources: vec![
                "Social Engineering: The Science of Human Hacking".to_string(),
                "SANS SEC567".to_string(),
            ],
        });

        // Add more security topics...
        map
    }

    fn init_gaming() -> HashMap<String, GamingInfo> {
        let mut map = HashMap::new();
        
        map.insert("overwatch2".to_string(), GamingInfo {
            game: "Overwatch 2".to_string(),
            genre: vec!["FPS".to_string(), "Team-based".to_string()],
            tips: vec![
                "Focus on positioning over aim".to_string(),
                "Group up before engaging".to_string(),
                "Counter-pick enemy composition".to_string(),
            ],
            meta_strategies: vec![
                "Dive comp with Winston/D.Va".to_string(),
                "Double shield with Orisa/Sigma".to_string(),
            ],
            patch_notes: vec![
                PatchNote {
                    version: "2.0.1".to_string(),
                    date: Utc::now(), // Example date
                    changes: vec!["Hero balance updates".to_string()],
                },
            ],
        });

        // Add more games...
        map
    }

    pub fn get_response(&mut self, query: &str) -> Option<String> {
        // First check cache
        if let Some(cached) = self.response_cache.get_mut(query) {
            cached.access_count += 1;
            return Some(cached.content.clone());
        }

        // Generate response based on query
        let response = self.generate_response(query)?;

        // Cache the response
        self.cache_response(query, &response);

        Some(response)
    }

    fn generate_response(&self, query: &str) -> Option<String> {
        // Check each knowledge base
        if let Some(toku) = self.tokusatsu.get(query) {
            return Some(format!("{}: {}. Fun fact: {}", 
                toku.title, 
                toku.description,
                toku.fun_facts[0]
            ));
        }

        if let Some(anime) = self.anime.get(query) {
            return Some(format!("{}: {}. If you like this, check out: {}", 
                anime.title,
                anime.synopsis,
                anime.recommendations.join(", ")
            ));
        }

        if let Some(security) = self.cybersecurity.get(query) {
            return Some(format!("{}: {}\nBest practices:\n- {}", 
                security.topic,
                security.description,
                security.best_practices.join("\n- ")
            ));
        }

        if let Some(game) = self.gaming.get(query) {
            return Some(format!("{} Tips:\n- {}", 
                game.game,
                game.tips.join("\n- ")
            ));
        }

        None
    }

    fn cache_response(&mut self, query: &str, response: &str) {
        self.response_cache.insert(query.to_string(), CachedResponse {
            content: response.to_string(),
            timestamp: Utc::now(),
            access_count: 1,
        });
    }

    pub fn get_random_fact(&self, category: &str) -> Option<String> {
        match category {
            "tokusatsu" => {
                let toku = self.tokusatsu.values().collect::<Vec<_>>();
                let random_toku = toku.get(fastrand::usize(..toku.len()))?;
                Some(random_toku.fun_facts[fastrand::usize(..random_toku.fun_facts.len())].clone())
            },
            "cybersecurity" => {
                let security = self.cybersecurity.values().collect::<Vec<_>>();
                let random_security = security.get(fastrand::usize(..security.len()))?;
                Some(random_security.best_practices[fastrand::usize(..random_security.best_practices.len())].clone())
            },
            _ => None,
        }
    }

    pub fn update_seasonal_anime(&mut self) {
        if let Some(anime) = self.anime.get_mut("current_season") {
            if let Some(seasonal) = &mut anime.seasonal_info {
                seasonal.currently_airing = true;
                // Update other seasonal information...
            }
        }
    }

    pub fn add_tokusatsu_info(&mut self, key: &str, info: TokusatsuInfo) {
        self.tokusatsu.insert(key.to_string(), info);
    }

    pub async fn update_tokusatsu_knowledge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut scraper = crate::scrapers::tokusatsu_scraper::TokuScraper::new();
        scraper.update_knowledge_base(self).await?;
        Ok(())
    }

    pub fn add_anime_info(&mut self, category: &str, info: AnimeInfo) {
        self.anime.insert(category.to_string(), info);
    }

    pub async fn update_anime_knowledge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut scraper = crate::scrapers::anime_scraper::AnimeNewsScraper::new();
        scraper.update_anime_knowledge(self).await?;
        Ok(())
    }

    pub async fn get_seasonal_anime(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut scraper = crate::scrapers::anime_scraper::AnimeNewsScraper::new();
        scraper.scrape_seasonal_anime().await
    }

    pub fn add_security_info(&mut self, category: &str, info: SecurityInfo) {
        self.cybersecurity.insert(category.to_string(), info);
    }

    pub async fn update_security_knowledge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut scraper = crate::scrapers::security_scraper::SecurityNewsScraper::new();
        scraper.update_security_knowledge(self).await?;
        Ok(())
    }

    pub async fn get_critical_vulnerabilities(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut scraper = crate::scrapers::security_scraper::SecurityNewsScraper::new();
        let critical_vulns = scraper.get_critical_vulnerabilities().await?;
        Ok(critical_vulns.into_iter().map(|article| article.title).collect())
    }

    pub async fn update_all_security_knowledge(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Update from Dark Reading
        let mut dark_reading = crate::scrapers::darkreading_scraper::DarkReadingScraper::new();
        dark_reading.update_security_knowledge(self).await?;

        // Update from CompTIA study guide
        let mut comptia = crate::scrapers::comptia_scraper::CompTIAScraper::new(
            "resources/CompTIA-Security-Study-Guide.pdf".to_string()
        );
        comptia.update_security_knowledge(self).await?;

        // Update from The Hacker News (existing)
        let mut hacker_news = crate::scrapers::security_scraper::SecurityNewsScraper::new();
        hacker_news.update_security_knowledge(self).await?;

        Ok(())
    }
} 