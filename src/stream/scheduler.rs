use chrono::{DateTime, Utc, Weekday, Duration, NaiveTime};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSchedule {
    pub weekly_schedule: HashMap<Weekday, Vec<ScheduledStream>>,
    pub special_events: Vec<SpecialEvent>,
    pub stream_duration: Duration,
    pub break_duration: Duration,
    pub preparation_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledStream {
    pub stream_type: StreamType,
    pub start_time: NaiveTime,
    pub title_template: String,
    pub tags: Vec<String>,
    pub segments: Vec<StreamSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StreamType {
    CyberSecurity,
    Gaming,
    Tokusatsu,
    AnimeDiscussion,
    TechTutorial,
    CasualChat,
    SpecialEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamSegment {
    pub name: String,
    pub duration: Duration,
    pub activity: String,
    pub required_resources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialEvent {
    pub name: String,
    pub date: DateTime<Utc>,
    pub stream_type: StreamType,
    pub description: String,
    pub duration: Duration,
}

pub struct StreamScheduler {
    schedule: StreamSchedule,
    next_stream: Option<ScheduledStream>,
    last_stream_type: Option<StreamType>,
}

impl StreamScheduler {
    pub fn new() -> Self {
        Self {
            schedule: Self::init_default_schedule(),
            next_stream: None,
            last_stream_type: None,
        }
    }

    fn init_default_schedule() -> StreamSchedule {
        let mut weekly_schedule = HashMap::new();

        // Monday - Cybersecurity Focus
        weekly_schedule.insert(Weekday::Mon, vec![
            ScheduledStream {
                stream_type: StreamType::CyberSecurity,
                start_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(), // 8 PM
                title_template: "🔒 Kawaii Hacker News & Security Updates! 💻".to_string(),
                tags: vec!["cybersecurity", "hacking", "tech news"].iter().map(String::from).collect(),
                segments: vec![
                    StreamSegment {
                        name: "Security News Recap".to_string(),
                        duration: Duration::hours(1),
                        activity: "Review latest security news and vulnerabilities".to_string(),
                        required_resources: vec!["security_news_scraper".to_string()],
                    },
                    StreamSegment {
                        name: "Live Security Analysis".to_string(),
                        duration: Duration::hours(1),
                        activity: "Analyze recent security incidents".to_string(),
                        required_resources: vec!["security_tools".to_string()],
                    },
                ],
            }
        ]);

        // Wednesday - Tokusatsu & Anime
        weekly_schedule.insert(Weekday::Wed, vec![
            ScheduledStream {
                stream_type: StreamType::Tokusatsu,
                start_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
                title_template: "✨ Toku Talk & Anime Updates! 🦹‍♀️".to_string(),
                tags: vec!["tokusatsu", "anime", "kamen rider"].iter().map(String::from).collect(),
                segments: vec![
                    StreamSegment {
                        name: "Weekly Toku Review".to_string(),
                        duration: Duration::minutes(90),
                        activity: "Discuss latest Kamen Rider/Sentai episodes".to_string(),
                        required_resources: vec!["toku_news_scraper".to_string()],
                    },
                    StreamSegment {
                        name: "Anime News & Discussion".to_string(),
                        duration: Duration::minutes(90),
                        activity: "Cover seasonal anime and industry news".to_string(),
                        required_resources: vec!["anime_news_scraper".to_string()],
                    },
                ],
            }
        ]);

        // Friday - Gaming Night
        weekly_schedule.insert(Weekday::Fri, vec![
            ScheduledStream {
                stream_type: StreamType::Gaming,
                start_time: NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
                title_template: "🎮 Friday Night Gaming! Let's rank up! 💪".to_string(),
                tags: vec!["gaming", "valorant", "apex"].iter().map(String::from).collect(),
                segments: vec![
                    StreamSegment {
                        name: "Warm Up Games".to_string(),
                        duration: Duration::minutes(30),
                        activity: "Practice aim and mechanics".to_string(),
                        required_resources: vec!["game_launcher".to_string()],
                    },
                    StreamSegment {
                        name: "Ranked Grind".to_string(),
                        duration: Duration::hours(2),
                        activity: "Competitive matches with viewers".to_string(),
                        required_resources: vec!["game_trainer".to_string()],
                    },
                ],
            }
        ]);

        // Saturday - Tech Talk & Gaming
        weekly_schedule.insert(Weekday::Sat, vec![
            ScheduledStream {
                stream_type: StreamType::TechTutorial,
                start_time: NaiveTime::from_hms_opt(15, 0, 0).unwrap(), // 3 PM
                title_template: "💻 Tech Talk Saturday & Chill Gaming! 🎮".to_string(),
                tags: vec!["tech talk", "cybersecurity", "gaming", "chill"].iter().map(String::from).collect(),
                segments: vec![
                    StreamSegment {
                        name: "Tech Talk & Discussion".to_string(),
                        duration: Duration::hours(2),
                        activity: "Discuss latest tech trends, news, and cybersecurity topics".to_string(),
                        required_resources: vec![
                            "tech_news_scraper".to_string(),
                            "security_knowledge_base".to_string(),
                            "presentation_tools".to_string()
                        ],
                    },
                    StreamSegment {
                        name: "Cozy Gaming".to_string(),
                        duration: Duration::hours(2),
                        activity: "Relaxed gaming session with chat".to_string(),
                        required_resources: vec![
                            "game_launcher".to_string(),
                            "stream_overlay".to_string()
                        ],
                    },
                ],
            }
        ]);

        StreamSchedule {
            weekly_schedule,
            special_events: Vec::new(),
            stream_duration: Duration::hours(3),
            break_duration: Duration::minutes(10),
            preparation_time: Duration::minutes(30),
        }
    }

    pub fn get_next_stream(&self) -> Option<&ScheduledStream> {
        let now = Utc::now();
        let current_weekday = now.weekday();
        let current_time = now.time();

        // First check today's streams
        if let Some(today_streams) = self.schedule.weekly_schedule.get(&current_weekday) {
            for stream in today_streams {
                if stream.start_time > current_time {
                    return Some(stream);
                }
            }
        }

        // Then check next days until we find a stream
        let mut next_day = current_weekday.succ();
        for _ in 0..7 {
            if let Some(streams) = self.schedule.weekly_schedule.get(&next_day) {
                if let Some(stream) = streams.first() {
                    return Some(stream);
                }
            }
            next_day = next_day.succ();
        }

        None
    }

    pub fn add_special_event(&mut self, event: SpecialEvent) {
        self.schedule.special_events.push(event);
        self.schedule.special_events.sort_by_key(|e| e.date);
    }

    pub fn get_stream_preparation_tasks(&self, stream_type: &StreamType) -> Vec<String> {
        match stream_type {
            StreamType::CyberSecurity => vec![
                "Update security news database".to_string(),
                "Prepare vulnerability demonstrations".to_string(),
                "Check latest CVE updates".to_string(),
            ],
            StreamType::Gaming => vec![
                "Warm up aim".to_string(),
                "Update game settings".to_string(),
                "Check for game updates".to_string(),
            ],
            StreamType::Tokusatsu => vec![
                "Watch latest episodes".to_string(),
                "Update toku news database".to_string(),
                "Prepare episode discussion points".to_string(),
            ],
            _ => vec![
                "Check stream settings".to_string(),
                "Test audio levels".to_string(),
                "Update stream title and tags".to_string(),
            ],
        }
    }

    pub fn generate_next_stream_title(&self) -> String {
        if let Some(next) = &self.next_stream {
            match next.stream_type {
                StreamType::CyberSecurity => format!("🔒 {} | Kawaii Hacker Time! 💻", next.title_template),
                StreamType::Gaming => format!("🎮 {} | Let's rank up together! ⚡", next.title_template),
                StreamType::Tokusatsu => format!("✨ {} | Henshin Time! 🦹‍♀️", next.title_template),
                StreamType::AnimeDiscussion => format!("🌸 {} | Anime & Chill! 🎌", next.title_template),
                _ => next.title_template.clone(),
            }
        } else {
            "🌟 Kawaii Stream Time! | Join the fun! ✨".to_string()
        }
    }

    pub fn get_required_resources(&self, stream_type: &StreamType) -> Vec<String> {
        let mut resources = vec![
            "obs_controller".to_string(),
            "chat_manager".to_string(),
            "voice_system".to_string(),
        ];

        match stream_type {
            StreamType::CyberSecurity => resources.extend(vec![
                "security_scraper".to_string(),
                "knowledge_base".to_string(),
                "defense_system".to_string(),
            ]),
            StreamType::Gaming => resources.extend(vec![
                "game_launcher".to_string(),
                "game_trainer".to_string(),
                "input_system".to_string(),
            ]),
            StreamType::Tokusatsu => resources.extend(vec![
                "toku_scraper".to_string(),
                "anime_scraper".to_string(),
            ]),
            _ => {}
        }

        resources
    }
} 