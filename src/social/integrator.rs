use twitter_v2::TwitterApi;
use instagram_private_api::Client as InstagramClient;
use serenity::voice::{self, AudioReceiver, AudioSource};
use crate::ai::{PersonalityCore, VoiceProcessor, SpeechSynthesis};
use tokio::sync::RwLock;

pub struct SocialMediaIntegrator {
    twitter: TwitterApi,
    instagram: InstagramClient,
    voice_processor: VoiceProcessor,
    speech_synthesis: SpeechSynthesis,
    personality: Arc<RwLock<PersonalityCore>>,
    call_context: CallContextManager,
}

struct CallContextManager {
    active_calls: HashMap<ChannelId, CallContext>,
    voice_settings: VoiceSettings,
    conversation_memory: ConversationMemory,
}

struct CallContext {
    participants: Vec<UserId>,
    topic: Option<String>,
    mood: CallMood,
    speaking_queue: VecDeque<UserId>,
    last_interaction: HashMap<UserId, DateTime<Utc>>,
}

impl SocialMediaIntegrator {
    pub async fn new(
        twitter_token: &str,
        instagram_creds: InstagramCredentials,
        personality: Arc<RwLock<PersonalityCore>>,
    ) -> Result<Self> {
        Ok(Self {
            twitter: TwitterApi::new(twitter_token),
            instagram: InstagramClient::new(instagram_creds)?,
            voice_processor: VoiceProcessor::new(),
            speech_synthesis: SpeechSynthesis::new(),
            personality,
            call_context: CallContextManager::new(),
        })
    }

    // Voice call handling
    pub async fn join_voice_call(&mut self, channel_id: ChannelId, guild_id: GuildId) -> Result<()> {
        let manager = voice::get_voice_manager(&guild_id).await?;
        let handler = manager.join(channel_id).await?;

        // Set up voice processing
        handler.add_global_event(
            voice::VoiceEvent::Speaking(|ss| {
                self.handle_voice_event(ss).await
            })
        );

        // Initialize call context
        self.call_context.initialize_call(channel_id).await?;

        Ok(())
    }

    async fn handle_voice_event(&mut self, speaking_state: voice::SpeakingState) -> Result<()> {
        let user_id = speaking_state.user_id;
        let channel_id = speaking_state.channel_id;
        
        if speaking_state.speaking {
            // Process incoming voice
            let audio = self.voice_processor.process_incoming_audio(speaking_state.audio_data).await?;
            
            // Convert speech to text
            let text = self.voice_processor.speech_to_text(audio).await?;
            
            // Generate response through personality core
            let response = self.personality.write().await
                .generate_voice_response(&text, &self.call_context.get_context(channel_id)).await?;
            
            // Convert response to speech
            let audio_response = self.speech_synthesis.generate_speech(&response).await?;
            
            // Queue response for natural timing
            self.call_context.queue_response(channel_id, audio_response).await?;
        }

        Ok(())
    }

    // Social media posting
    pub async fn post_stream_highlight(&self, clip: StreamClip) -> Result<()> {
        // Process clip for different platforms
        let (twitter_video, instagram_reel) = self.process_clip_for_platforms(&clip).await?;

        // Post to Twitter
        let tweet = self.create_engaging_tweet(&clip).await?;
        self.twitter.tweet()
            .media(twitter_video)
            .text(&tweet)
            .send()
            .await?;

        // Post to Instagram
        let instagram_caption = self.create_instagram_caption(&clip).await?;
        self.instagram.upload_reel(instagram_reel, &instagram_caption).await?;

        Ok(())
    }

    async fn create_engaging_tweet(&self, clip: &StreamClip) -> Result<String> {
        let personality = self.personality.read().await;
        let context = clip.get_context();
        
        // Generate engaging tweet text
        let tweet = personality.generate_social_post(
            Platform::Twitter,
            context,
            MAX_TWEET_LENGTH
        ).await?;

        Ok(tweet)
    }

    async fn process_clip_for_platforms(&self, clip: &StreamClip) -> Result<(VideoFile, VideoFile)> {
        // Process for Twitter
        let twitter_video = VideoProcessor::new()
            .optimize_for_twitter(clip.video_data())
            .add_branding()
            .process()
            .await?;

        // Process for Instagram
        let instagram_video = VideoProcessor::new()
            .optimize_for_instagram_reels(clip.video_data())
            .add_branding()
            .process()
            .await?;

        Ok((twitter_video, instagram_video))
    }
}

impl CallContextManager {
    async fn initialize_call(&mut self, channel_id: ChannelId) -> Result<()> {
        let context = CallContext {
            participants: Vec::new(),
            topic: None,
            mood: CallMood::default(),
            speaking_queue: VecDeque::new(),
            last_interaction: HashMap::new(),
        };

        self.active_calls.insert(channel_id, context);
        Ok(())
    }

    async fn queue_response(&mut self, channel_id: ChannelId, response: AudioData) -> Result<()> {
        let context = self.active_calls.get_mut(&channel_id)
            .ok_or(Error::NoActiveCall)?;

        // Add natural delay based on conversation flow
        let delay = self.calculate_natural_delay(context);
        tokio::time::sleep(delay).await;

        // Queue response with appropriate timing
        context.speaking_queue.push_back(response);
        
        Ok(())
    }

    fn calculate_natural_delay(&self, context: &CallContext) -> Duration {
        let base_delay = Duration::from_millis(800); // Base thinking time
        
        // Adjust based on conversation mood and pace
        let mood_factor = match context.mood {
            CallMood::Excited => 0.7,
            CallMood::Casual => 1.0,
            CallMood::Serious => 1.3,
        };

        // Add slight randomness for natural feel
        let random_factor = rand::thread_rng().gen_range(0.9..1.1);
        
        base_delay.mul_f64(mood_factor * random_factor)
    }
}

// Add to DiscordManager
impl DiscordManager {
    pub async fn handle_voice_state_update(
        &mut self,
        old: Option<VoiceState>,
        new: VoiceState,
    ) -> Result<()> {
        // Handle users joining/leaving voice channels
        if let Some(channel_id) = new.channel_id {
            if old.and_then(|s| s.channel_id) != Some(channel_id) {
                // User joined a new channel
                self.handle_user_joined_voice(channel_id, new.user_id).await?;
            }
        } else if let Some(old) = old {
            // User left a channel
            self.handle_user_left_voice(old.channel_id.unwrap(), new.user_id).await?;
        }

        Ok(())
    }

    async fn handle_user_joined_voice(&mut self, channel_id: ChannelId, user_id: UserId) -> Result<()> {
        // Check if we should join the call
        if self.should_join_call(channel_id).await? {
            self.social_integrator.join_voice_call(channel_id, self.guild_id).await?;
        }

        Ok(())
    }
} 