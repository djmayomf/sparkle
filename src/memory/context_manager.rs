pub struct ContextManager {
    short_term_memory: VecDeque<Interaction>,
    long_term_memory: HashMap<String, UserProfile>,
    conversation_context: HashMap<String, Vec<String>>,
    recent_topics: VecDeque<String>,
}

impl ContextManager {
    pub fn remember_interaction(&mut self, user: &str, message: &str) {
        // Store recent interactions
        self.short_term_memory.push_back(Interaction {
            user: user.to_string(),
            message: message.to_string(),
            timestamp: Utc::now(),
        });

        // Maintain conversation context
        if let Some(topics) = self.conversation_context.get_mut(user) {
            topics.push(message.to_string());
            if topics.len() > 10 {
                topics.remove(0);
            }
        }

        // Update user profile
        self.update_user_profile(user, message);
    }

    fn update_user_profile(&mut self, user: &str, message: &str) {
        let profile = self.long_term_memory
            .entry(user.to_string())
            .or_insert_with(UserProfile::new);

        profile.interaction_count += 1;
        profile.last_seen = Utc::now();
        
        // Remember user preferences and topics
        if message.contains("favorite") {
            profile.preferences.push(message.to_string());
        }
    }
} 