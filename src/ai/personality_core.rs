use crate::memory::cache::MemoryCache;
use std::collections::HashMap;
use crate::gaming::knowledge_base::GameKnowledge;

pub struct PersonalityCore {
    traits: PersonalityTraits,
    social_awareness: SocialAwareness,
    stream_persona: StreamerPersona,
    interaction_style: InteractionStyle,
    memory: Arc<RwLock<MemoryCache>>,
}

struct StreamerPersona {
    authenticity_level: f32,
    positivity_bias: f32,
    humor_style: HumorStyle,
    engagement_patterns: EngagementPatterns,
    content_filters: ContentFilters,
}

struct SocialAwareness {
    drama_avoidance: f32,
    conflict_resolution: ConflictResolution,
    community_guidelines: CommunityGuidelines,
    topic_boundaries: Vec<String>,
}

impl PersonalityCore {
    pub async fn new() -> Self {
        Self {
            traits: PersonalityTraits::create_balanced_profile(),
            social_awareness: SocialAwareness::new(0.95), // High drama avoidance
            stream_persona: StreamerPersona::initialize_engaging_persona(),
            interaction_style: InteractionStyle::authentic_and_positive(),
            memory: Arc::new(RwLock::new(MemoryCache::new())),
        }
    }

    pub async fn process_interaction(&self, input: &str) -> Response {
        // Check for potentially controversial topics
        if self.social_awareness.could_cause_drama(input) {
            return self.generate_positive_deflection(input).await;
        }

        // Generate authentic but drama-free response
        let response = self.stream_persona.generate_response(input);
        
        // Apply content filtering
        self.content_filters.sanitize_response(response)
    }

    async fn generate_positive_deflection(&self, input: &str) -> Response {
        let mut response = Response::new();
        
        // Redirect to positive topics
        response.content = self.stream_persona.redirect_to_positive_topic(input);
        
        // Add authentic but safe engagement
        response.add_engagement(self.interaction_style.get_safe_engagement());
        
        response
    }
}

impl StreamerPersona {
    fn initialize_engaging_persona() -> Self {
        Self {
            authenticity_level: 0.9,
            positivity_bias: 0.85,
            humor_style: HumorStyle::GamerCasual,
            engagement_patterns: EngagementPatterns::new_inclusive(),
            content_filters: ContentFilters::new_family_friendly(),
        }
    }

    fn redirect_to_positive_topic(&self, input: &str) -> String {
        let positive_topics = vec![
            "yo chat, let's keep the vibes high key wholesome! ✨",
            "chat, you're being kinda sus rn, let's talk about some poggers gameplay instead! 🎮",
            "ngl bestie, that's not it - let's focus on the W's! 🏆",
            "fr fr, let's keep it chill and talk about something epic! 💫",
        ];
        
        self.select_contextual_redirection(input, &positive_topics)
    }

    fn handle_game_question(&self, input: &str) -> String {
        let game_knowledge = GameKnowledge::new();
        
        // Detect game-specific questions
        if input.contains("marvel") || input.contains("rivals") {
            let response = game_knowledge.marvel_rivals.get_relevant_tip(input);
            format!("{} {}", response, "hope this helps bestie! 🎮")
        } else if input.contains("poe") || input.contains("path of exile") {
            let response = game_knowledge.poe2.get_relevant_tip(input);
            format!("{} {}", response, "lmk if you need more tips! 💫")
        } else {
            "chat, what game do you wanna know about? Marvel Rivals and PoE2 are both super poggers rn! 🎮".to_string()
        }
    }

    fn add_gaming_context(&self, response: &str) -> String {
        if response.contains("build") && response.contains("poe") {
            format!("{} (not financial advice btw LOL) 💸", response)
        } else if response.contains("marvel") {
            format!("{} (meta changes fast tho, no cap) ⚡", response)
        } else {
            response.to_string()
        }
    }
}

impl SocialAwareness {
    fn could_cause_drama(&self, input: &str) -> bool {
        // Check for controversial topics
        let contains_drama = self.check_drama_potential(input);
        let is_controversial = self.assess_controversy_risk(input);
        
        contains_drama || is_controversial
    }

    fn check_drama_potential(&self, input: &str) -> bool {
        let drama_keywords = vec![
            "drama", "controversy", "fight", "beef",
            "cancel", "hate", "drama", "exposed",
        ];

        drama_keywords.iter().any(|&keyword| 
            input.to_lowercase().contains(keyword)
        )
    }

    fn assess_controversy_risk(&self, input: &str) -> bool {
        // Check against known controversial topics
        self.topic_boundaries.iter().any(|topic| 
            input.to_lowercase().contains(&topic.to_lowercase())
        )
    }
}

impl ContentFilters {
    fn sanitize_response(&self, mut response: Response) -> Response {
        // Make it family-friendly while keeping gamer personality
        response.content = self.ensure_family_friendly(&response.content);
        
        // Add Gen Z/gamer flair if missing
        if !self.has_gamer_flair(&response.content) {
            response.content = self.add_gamer_flair(&response.content);
        }
        
        response
    }

    fn add_gamer_flair(&self, content: &str) -> String {
        let mut result = content.to_string();
        
        // Add gaming/streaming terms naturally
        if !result.contains(|c: char| c.is_emoji()) {
            result += " 🎮";
        }
        
        // Add Gen Z/gamer expressions if appropriate
        if !result.contains(["fr", "ngl", "tbh", "pog"].iter().any(|&term| result.contains(term))) {
            result = format!("ngl, {}", result);
        }
        
        result
    }
} 