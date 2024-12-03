use pyo3::prelude::*;
use pyo3::types::PyDict;
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use rust_bert::RustBertError;
use tch::Device;
use rand::Rng;
use std::env;
mod twitch;
use twitch::TwitchAPI;
use chrono::{DateTime, Utc};
use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use std::collections::HashMap;

struct StreamerInfo {
    username: String,
    display_name: String,
    stream_title: String,
    category: String,
    uptime_minutes: i32,
    is_live: bool,
}

struct StreamInfo {
    streamer_name: String,
    streamer_id: String,
    current_activity: String,
    stream_category: String,
    stream_tags: Vec<String>,
    auto_follow_raiders: bool,
}

struct RaiderInfo {
    basic_info: StreamerInfo,
    description: String,
    profile_image: String,
    broadcaster_type: String,
    last_game: String,
}

#[derive(Debug)]
struct StreamerPersonality {
    interests: Vec<String>,
    streaming_style: String,
    favorite_games: Vec<String>,
    recent_achievements: Vec<String>,
    community_values: Vec<String>,
}

#[derive(Debug)]
enum RaidQuestion {
    AboutStream,
    AboutGame,
    AboutCommunity,
    AboutSchedule,
    General,
}

#[derive(Debug)]
enum ChatContext {
    Initial,
    Conversation,
    Engagement,
    Farewell,
}

#[derive(Debug)]
struct ChatTiming {
    last_message: DateTime<Utc>,
    min_gap: std::time::Duration,
    raid_duration: std::time::Duration,
}

#[derive(Debug)]
enum ChatEvent {
    Question(RaidQuestion),
    Excitement(ExcitingMoment),
    Achievement(String),
    Emote(String),
    Conversation(String),
    None,
}

#[derive(Debug)]
enum ExcitingMoment {
    GameWin,
    ClutchPlay,
    FunnyMoment,
    SkillDisplay,
    CommunityMoment,
}

#[derive(Debug)]
struct ChatState {
    last_emote_time: DateTime<Utc>,
    conversation_topics: Vec<String>,
    emote_count: i32,
    has_introduced: bool,
}

#[derive(Component)]
struct AutoMove {
    timer: Timer,
    direction: Vec3,
}

#[derive(Resource)]
struct MovementSettings {
    speed: f32,
    change_direction_interval: f32,
    movement_range: f32,
}

#[derive(Component)]
struct CameraBox {
    position: Vec3,
    dimensions: Vec3,
}

#[derive(Component)]
struct ModelAnimation {
    idle_timer: Timer,
    gesture_timer: Timer,
    current_state: AnimationState,
}

#[derive(Debug)]
enum AnimationState {
    Idle,
    Walking,
    Talking,
    Waving,
    Pointing,
    Reacting,
}

#[derive(Resource)]
struct AnimationSettings {
    idle_duration: f32,
    gesture_frequency: f32,
    scale_range: (f32, f32),
    movement_smoothness: f32,
}

#[derive(Debug, Clone)]
struct EmotionalState {
    happiness: f32,
    energy: f32,
    engagement: f32,
    confidence: f32,
}

#[derive(Debug)]
struct BehaviorContext {
    recent_interactions: Vec<Interaction>,
    emotional_state: EmotionalState,
    attention_focus: AttentionPoint,
    personality_traits: PersonalityTraits,
}

#[derive(Debug, Clone)]
struct Interaction {
    timestamp: DateTime<Utc>,
    interaction_type: InteractionType,
    emotional_impact: f32,
}

#[derive(Debug, Clone)]
enum InteractionType {
    ChatMessage(String),
    ViewerReaction(String),
    StreamEvent(String),
    EnvironmentalChange(String),
}

#[derive(Debug, Clone)]
struct AttentionPoint {
    target: String,
    intensity: f32,
    duration: std::time::Duration,
}

#[derive(Debug, Clone)]
struct PersonalityTraits {
    extraversion: f32,
    adaptability: f32,
    empathy: f32,
    creativity: f32,
}

#[derive(Debug, Clone)]
struct CognitiveCapabilities {
    iq_level: f32,                    // Base IQ score (140-180 range)
    learning_rate: f32,               // Speed of knowledge acquisition
    pattern_recognition: f32,         // Pattern analysis capability
    problem_solving: f32,             // Complex problem-solving ability
    memory_capacity: f32,             // Information retention and recall
    processing_speed: f32,            // Mental processing velocity
}

#[derive(Debug, Clone)]
struct CyberSecurityExpertise {
    domains: HashMap<SecurityDomain, f32>,
    certifications: Vec<String>,
    threat_analysis_capability: f32,
    incident_response_skill: f32,
    zero_day_detection: f32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum SecurityDomain {
    NetworkSecurity,
    ApplicationSecurity,
    CloudSecurity,
    BlockchainSecurity,
    AISecuritySystems,
    QuantumCryptography,
    ZeroTrustArchitecture,
    AdvancedPenetrationTesting,
    ThreatIntelligence,
    IncidentResponse,
}

fn generate_text_rust() -> Result<String, RustBertError> {
    let model = TextGenerationModel::new(Default::default())?;
    let config = TextGenerationConfig {
        model_type: "gpt-neo".to_string(),
        model_name: "EleutherAI/gpt-neo-2.7B".to_string(),
        min_length: Some(50),
        max_length: Some(100),
        do_sample: true,
        temperature: Some(0.7),
        num_return_sequences: Some(1),
        device: if tch::Cuda::is_available() { Device::Cuda(0) } else { Device::Cpu },
        ..Default::default()
    };

    let input_text = "EleutherAI has";
    let output = model.generate(&[input_text], config)?;
    Ok(output[0].clone())
}

fn generate_text_python() -> PyResult<String> {
    Python::with_gil(|py| {
        // Import required modules
        let transformers = py.import("transformers")?;
        
        // Create pipeline with more explicit kwargs
        let kwargs = PyDict::new(py);
        kwargs.set_item("task", "text-generation")?;
        kwargs.set_item("model", "EleutherAI/gpt-neo-2.7B")?;
        kwargs.set_item("device", if tch::Cuda::is_available() { 0 } else { -1 })?;
        
        let generator = transformers.getattr("pipeline")?.call((), Some(kwargs))?;

        // Generation kwargs
        let gen_kwargs = PyDict::new(py);
        gen_kwargs.set_item("do_sample", true)?;
        gen_kwargs.set_item("min_length", 50)?;
        gen_kwargs.set_item("max_length", 100)?;
        gen_kwargs.set_item("temperature", 0.7)?;
        
        let result = generator.call(("EleutherAI has",), Some(gen_kwargs))?;
        let output = result.get_item(0)?.get_item("generated_text")?.extract::<String>()?;
        
        Ok(output)
    })
}

fn generate_incoming_raid_message(
    raider: &StreamerInfo,
    stream_info: &StreamInfo,
) -> Result<String, Box<dyn std::error::Error>> {
    // Auto-follow the raider (you'd implement this with your Twitch API client)
    if stream_info.auto_follow_raiders {
        println!("🤖 Auto-following raider: {}", raider.username);
        // follow_user(&raider.username)?;
    }

    let prompt = format!(
        "Thank you {} for the amazing raid! 💜 Welcome raiders! I'm {}, and we're currently {}! \
        Make sure to give {} a follow - they were just doing some awesome {}! \
        Quick intro: I'm a {} streamer who {}. EleutherAI has",
        raider.display_name,
        stream_info.streamer_name,
        stream_info.current_activity,
        raider.display_name,
        raider.stream_title,
        stream_info.stream_category,
        stream_info.stream_tags.join(", ")
    );

    generate_text_with_fallback(&prompt)
}

fn select_raid_target(
    potential_targets: &[StreamerInfo]
) -> Option<&StreamerInfo> {
    potential_targets
        .iter()
        .filter(|streamer| {
            streamer.is_live && 
            streamer.uptime_minutes >= 30
        })
        .max_by_key(|streamer| streamer.uptime_minutes)
}

fn generate_outgoing_raid_message(
    target: &StreamerInfo,
) -> Result<String, Box<dyn std::error::Error>> {
    let raid_phrases = [
        "Let's share some love with",
        "Time to bring our amazing community to",
        "Get ready for an epic raid to",
        "Let's make a new friend! Raiding",
        "Community vibes incoming! Heading to",
        "Bringing the party to",
    ];
    
    let random_phrase = raid_phrases[rand::thread_rng().gen_range(0..raid_phrases.len())];
    
    // Auto-follow the target (you'd implement this with your Twitch API client)
    println!("🤖 Auto-following raid target: {}", target.username);
    // follow_user(&target.username)?;
    
    let prompt = format!(
        "{} {}! They're doing some awesome {} in {}! \
        I picked them because their stream looks super engaging! \
        Let's show them what our community's love looks like! EleutherAI has",
        random_phrase,
        target.display_name,
        target.stream_title,
        target.category
    );

    generate_text_with_fallback(&prompt)
}

fn generate_text_with_fallback(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    match generate_text_with_prompt(prompt) {
        Ok(output) => Ok(output),
        Err(e) => {
            println!("Rust generation failed ({}), falling back to Python...", e);
            generate_text_python_with_prompt(prompt)
        }
    }
}

fn generate_text_with_prompt(prompt: &str) -> Result<String, RustBertError> {
    let model = TextGenerationModel::new(Default::default())?;
    let config = TextGenerationConfig {
        model_type: "gpt-neo".to_string(),
        model_name: "EleutherAI/gpt-neo-2.7B".to_string(),
        min_length: Some(50),
        max_length: Some(100),
        do_sample: true,
        temperature: Some(0.7),
        num_return_sequences: Some(1),
        device: if tch::Cuda::is_available() { Device::Cuda(0) } else { Device::Cpu },
        ..Default::default()
    };

    let output = model.generate(&[prompt], config)?;
    Ok(output[0].clone())
}

fn generate_text_python_with_prompt(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    Python::with_gil(|py| {
        let transformers = py.import("transformers")?;
        
        let kwargs = PyDict::new(py);
        kwargs.set_item("task", "text-generation")?;
        kwargs.set_item("model", "EleutherAI/gpt-neo-2.7B")?;
        kwargs.set_item("device", if tch::Cuda::is_available() { 0 } else { -1 })?;
        
        let generator = transformers.getattr("pipeline")?.call((), Some(kwargs))?;

        let gen_kwargs = PyDict::new(py);
        gen_kwargs.set_item("do_sample", true)?;
        gen_kwargs.set_item("min_length", 50)?;
        gen_kwargs.set_item("max_length", 100)?;
        gen_kwargs.set_item("temperature", 0.7)?;
        
        let result = generator.call((prompt,), Some(gen_kwargs))?;
        let output = result.get_item(0)?.get_item("generated_text")?.extract::<String>()?;
        
        Ok(output)
    })
}

async fn handle_raid(
    api: &TwitchAPI,
    stream_info: &StreamInfo,
    raider: &StreamerInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    // Auto-follow the raider
    if stream_info.auto_follow_raiders {
        api.follow_user(&stream_info.streamer_id, &raider.username).await?;
    }

    // Get detailed raider info
    let user_info = api.get_user_info(&raider.username).await?;
    let last_game = api.get_channel_info(&user_info.id).await?;

    let raider_info = RaiderInfo {
        basic_info: raider.clone(),
        description: user_info.description,
        profile_image: user_info.profile_image_url,
        broadcaster_type: user_info.broadcaster_type,
        last_game,
    };

    // Generate multiple supportive messages
    let messages = generate_raid_support_messages(&raider_info, stream_info)?;
    
    // Print messages with timing gaps (in a real implementation, you'd send these to chat)
    for message in messages {
        println!("{}", message);
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    Ok(())
}

fn generate_raid_support_messages(
    raider: &RaiderInfo,
    stream_info: &StreamInfo,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut messages = Vec::new();

    // Initial raid welcome
    let welcome = format!(
        "🎉 RAID ALERT! 🎉 Welcome {} and their amazing community! Thank you for the raid! 💜",
        raider.basic_info.display_name
    );
    messages.push(welcome);

    // Raider info and shoutout
    let shoutout = format!(
        "Everyone, {} is an awesome {} streamer who was just playing {}! \
        Go give them a follow at twitch.tv/{} 💜",
        raider.basic_info.display_name,
        raider.last_game,
        raider.basic_info.stream_title,
        raider.basic_info.username
    );
    messages.push(shoutout);

    // If they have a description, share it
    if !raider.description.is_empty() {
        let about = format!(
            "A bit about {}: {} 💜",
            raider.basic_info.display_name,
            raider.description
        );
        messages.push(about);
    }

    // Stream recap
    let recap = format!(
        "For our raiders: I'm {}, and we're currently {}! \
        I'm a {} streamer who {}. Welcome to the community! 💜",
        stream_info.streamer_name,
        stream_info.current_activity,
        stream_info.stream_category,
        stream_info.stream_tags.join(", ")
    );
    messages.push(recap);

    // Generate an AI message for extra personalization
    let ai_prompt = format!(
        "Create an enthusiastic and welcoming message for {} who just raided the channel \
        after streaming {}. They are known for {}. EleutherAI has",
        raider.basic_info.display_name,
        raider.basic_info.stream_title,
        raider.description
    );

    let ai_message = generate_text_with_fallback(&ai_prompt)?;
    messages.push(ai_message);

    Ok(messages)
}

fn generate_raid_response(
    stream_info: &StreamInfo,
    personality: &StreamerPersonality,
    question_type: RaidQuestion,
) -> Result<String, Box<dyn std::error::Error>> {
    let response_prompt = match question_type {
        RaidQuestion::AboutStream => format!(
            "As a streamer who {}, respond naturally to someone asking what your stream is about. \
            Include that you're currently {} and typically {}. \
            Keep it casual and friendly, like chatting with a friend. EleutherAI has",
            personality.streaming_style,
            stream_info.current_activity,
            stream_info.stream_tags.join(", ")
        ),
        RaidQuestion::AboutGame => format!(
            "Respond enthusiastically about why you're playing {} right now. \
            Mention that you {} and include one of your recent achievements: {}. \
            Keep it casual and friendly. EleutherAI has",
            stream_info.current_activity,
            personality.streaming_style,
            personality.recent_achievements.join(", ")
        ),
        RaidQuestion::AboutCommunity => format!(
            "Share what makes your community special! Mention that you value {} \
            and your community loves {}. Keep it warm and welcoming. EleutherAI has",
            personality.community_values.join(", "),
            stream_info.stream_tags.join(", ")
        ),
        RaidQuestion::AboutSchedule => format!(
            "Share your typical streaming schedule in a friendly way, mentioning that \
            you love {} and usually play {}. EleutherAI has",
            personality.streaming_style,
            personality.favorite_games.join(", ")
        ),
        RaidQuestion::General => format!(
            "Share a friendly greeting about your stream, mentioning that you're {} \
            and love {}. Keep it casual and welcoming. EleutherAI has",
            stream_info.current_activity,
            personality.interests.join(", ")
        ),
    };

    generate_text_with_fallback(&response_prompt)
}

async fn maintain_raid_presence(
    api: &TwitchAPI,
    stream_info: &StreamInfo,
    personality: &StreamerPersonality,
    target: &StreamerInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let timing = ChatTiming {
        last_message: Utc::now(),
        min_gap: std::time::Duration::from_secs(30),
        raid_duration: std::time::Duration::from_secs(15 * 60),
    };

    let mut state = ChatState {
        last_emote_time: Utc::now(),
        conversation_topics: Vec::new(),
        emote_count: 0,
        has_introduced: false,
    };

    // Send initial raid message
    let message = generate_outgoing_raid_message(target)?;
    println!("Initial Raid Message: {}", message);

    let start_time = Utc::now();
    let mut context = ChatContext::Initial;

    while Utc::now().signed_duration_since(start_time) < timing.raid_duration {
        // Reset emote count periodically
        if Utc::now().signed_duration_since(state.last_emote_time) > chrono::Duration::minutes(5) {
            state.emote_count = 0;
        }

        // Vary wait time based on context
        let wait_time = match context {
            ChatContext::Initial => rand::thread_rng().gen_range(30..45),
            ChatContext::Conversation => rand::thread_rng().gen_range(15..30),
            ChatContext::Engagement => rand::thread_rng().gen_range(20..40),
            ChatContext::Farewell => rand::thread_rng().gen_range(45..60),
        };
        tokio::time::sleep(std::time::Duration::from_secs(wait_time)).await;

        // Simulate chat monitoring and respond naturally
        if let Some(chat_event) = simulate_chat_event(target) {
            match chat_event {
                ChatEvent::Question(question_type) => {
                    if should_engage_naturally(&timing, &state, &context) {
                        let response = generate_raid_response(
                            stream_info,
                            personality,
                            question_type,
                            &context,
                        )?;
                        println!("Natural Response: {}", response);
                        context = ChatContext::Conversation;
                    }
                },
                ChatEvent::Excitement(moment) => {
                    if should_engage_naturally(&timing, &state, &context) {
                        let response = generate_excitement_response(moment, &context)?;
                        println!("Reaction: {}", response);
                        context = ChatContext::Engagement;
                    }
                },
                ChatEvent::Achievement(achievement) => {
                    if should_engage_naturally(&timing, &state, &context) {
                        let response = format!("Congratulations on {}! That's awesome! 🎉", achievement);
                        println!("Achievement Response: {}", response);
                    }
                },
                ChatEvent::Emote(emote) => {
                    if let Some(response) = generate_emote_response(&emote, &mut state) {
                        println!("Emote Response: {}", response);
                    }
                },
                ChatEvent::Conversation(topic) => {
                    if should_engage_naturally(&timing, &state, &context) 
                        && !state.conversation_topics.contains(&topic) {
                        state.conversation_topics.push(topic.clone());
                        let response = generate_natural_engagement(target, &context, personality)?;
                        println!("Conversation Response: {}", response);
                        context = ChatContext::Engagement;
                    }
                },
                ChatEvent::None => {
                    // Occasionally engage naturally if there's been a long silence
                    if should_engage_naturally(&timing, &state, &context) {
                        let response = generate_natural_engagement(target, &context, personality)?;
                        println!("Natural Engagement: {}", response);
                        context = ChatContext::Engagement;
                    }
                }
            }
        }
    }

    // Send a friendly goodbye message
    let farewell = generate_farewell_message(target, personality)?;
    println!("Farewell Message: {}", farewell);

    Ok(())
}

fn generate_natural_engagement(
    target: &StreamerInfo,
    context: &ChatContext,
    personality: &StreamerPersonality,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = match context {
        ChatContext::Initial | ChatContext::Conversation => format!(
            "React naturally to something cool happening in {}'s stream, \
            mentioning that you {} and love this kind of content. \
            Keep it casual and genuine. EleutherAI has",
            target.display_name,
            personality.streaming_style
        ),
        ChatContext::Engagement => format!(
            "Share a genuine observation or question about {}'s gameplay or stream, \
            relating it to your experience with {}. \
            Keep it conversational. EleutherAI has",
            target.display_name,
            personality.favorite_games.join(" or ")
        ),
        ChatContext::Farewell => format!(
            "Express genuine enjoyment of the stream and mention needing to go, \
            keeping it natural and friendly. EleutherAI has"
        ),
    };

    generate_text_with_fallback(&prompt)
}

fn generate_farewell_message(
    target: &StreamerInfo,
    personality: &StreamerPersonality,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!(
        "Say goodbye naturally after enjoying {}'s stream for a while, \
        mentioning that you {} and really enjoyed their content. \
        Keep it genuine and friendly. EleutherAI has",
        target.display_name,
        personality.streaming_style
    );

    generate_text_with_fallback(&prompt)
}

async fn select_and_raid(
    api: &TwitchAPI,
    stream_info: &StreamInfo,
    personality: &StreamerPersonality,
) -> Result<(), Box<dyn std::error::Error>> {
    let potential_targets = api.get_live_streams(None).await?;
    
    if let Some(target) = select_raid_target(&potential_targets) {
        api.follow_user(&stream_info.streamer_id, &target.username).await?;
        
        // Maintain natural presence in the raided channel
        maintain_raid_presence(api, stream_info, personality, target).await?;
    } else {
        println!("No suitable raid targets found at this time.");
    }
    
    Ok(())
}

// Add this function to simulate natural chat behavior
fn should_engage_naturally(timing: &ChatTiming, state: &ChatState, context: &ChatContext) -> bool {
    let time_since_last = Utc::now().signed_duration_since(timing.last_message);
    
    match context {
        ChatContext::Initial => time_since_last > chrono::Duration::seconds(45),
        ChatContext::Conversation => time_since_last > chrono::Duration::seconds(20),
        ChatContext::Engagement => time_since_last > chrono::Duration::seconds(30),
        ChatContext::Farewell => time_since_last > chrono::Duration::seconds(60),
    }
}

// Add this function to generate reactions to exciting moments
fn generate_excitement_response(
    moment: ExcitingMoment,
    context: &ChatContext,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = match moment {
        ExcitingMoment::GameWin => 
            "React naturally and excitedly to someone winning their game, \
            using appropriate emotes and genuine enthusiasm. EleutherAI has",
        ExcitingMoment::ClutchPlay =>
            "React with genuine amazement to an incredible clutch play, \
            using hype emotes and authentic excitement. EleutherAI has",
        ExcitingMoment::FunnyMoment =>
            "React naturally to a funny moment in stream, \
            using laugh emotes and showing genuine amusement. EleutherAI has",
        ExcitingMoment::SkillDisplay =>
            "React with genuine appreciation to an impressive display of skill, \
            using supportive emotes and authentic praise. EleutherAI has",
        ExcitingMoment::CommunityMoment =>
            "React warmly to a wholesome community moment, \
            using heart emotes and genuine appreciation. EleutherAI has",
    };

    generate_text_with_fallback(prompt)
}

// Add this function to handle emote usage
fn generate_emote_response(
    emote: &str,
    state: &mut ChatState,
) -> Option<String> {
    // Don't spam emotes
    if state.emote_count > 3 || 
       Utc::now().signed_duration_since(state.last_emote_time) < chrono::Duration::seconds(30) {
        return None;
    }

    state.emote_count += 1;
    state.last_emote_time = Utc::now();

    Some(emote.to_string())
}

#[derive(Component)]
struct Player;

fn player_movement(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let movement_speed = 5.0;

    if keyboard.pressed(KeyCode::W) {
        player_transform.translation.z -= movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        player_transform.translation.z += movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        player_transform.translation.x -= movement_speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        player_transform.translation.x += movement_speed * time.delta_seconds();
    }
}

// Add this new system for autonomous movement
fn auto_movement_system(
    time: Res<Time>,
    movement_settings: Res<MovementSettings>,
    mut query: Query<(&mut Transform, &mut AutoMove), With<Player>>,
) {
    for (mut transform, mut auto_move) in query.iter_mut() {
        // Update timer
        if auto_move.timer.tick(time.delta()).just_finished() {
            // Generate new random direction
            let mut rng = rand::thread_rng();
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            auto_move.direction = Vec3::new(
                angle.cos(),
                0.0,
                angle.sin(),
            ).normalize();
        }

        // Move in current direction
        let movement = auto_move.direction * movement_settings.speed * time.delta_seconds();
        let new_pos = transform.translation + movement;

        // Keep within bounds
        let range = movement_settings.movement_range;
        if new_pos.x.abs() <= range && new_pos.z.abs() <= range {
            transform.translation = new_pos;
        } else {
            // If hitting boundary, reverse direction
            auto_move.direction = -auto_move.direction;
        }

        // Smoothly rotate to face movement direction
        if movement.length_squared() > 0.0 {
            let target_rotation = Quat::from_rotation_y(auto_move.direction.z.atan2(auto_move.direction.x));
            transform.rotation = transform.rotation.slerp(target_rotation, 0.1);
        }
    }
}

// Add this system for natural idle animations and gestures
fn model_animation_system(
    time: Res<Time>,
    animation_settings: Res<AnimationSettings>,
    mut query: Query<(&mut Transform, &mut ModelAnimation), With<Player>>,
) {
    for (mut transform, mut animation) in query.iter_mut() {
        // Update timers
        if animation.idle_timer.tick(time.delta()).just_finished() {
            // Subtle idle movement
            let mut rng = rand::thread_rng();
            let idle_offset = Vec3::new(
                rng.gen_range(-0.1..0.1),
                rng.gen_range(-0.05..0.05),
                rng.gen_range(-0.1..0.1)
            );
            transform.translation += idle_offset;
        }

        // Random gestures
        if animation.gesture_timer.tick(time.delta()).just_finished() {
            let mut rng = rand::thread_rng();
            animation.current_state = match rng.gen_range(0..5) {
                0 => AnimationState::Waving,
                1 => AnimationState::Talking,
                2 => AnimationState::Pointing,
                3 => AnimationState::Reacting,
                _ => AnimationState::Idle,
            };
        }

        // Apply animation state
        match animation.current_state {
            AnimationState::Idle => {
                // Subtle breathing animation
                let breathing = (time.elapsed_seconds() * 2.0).sin() * 0.02;
                transform.scale = Vec3::new(1.0, 1.0 + breathing, 1.0);
            },
            AnimationState::Talking => {
                // Subtle head and hand movements
                let talk_motion = (time.elapsed_seconds() * 8.0).sin() * 0.05;
                transform.rotation *= Quat::from_rotation_y(talk_motion);
            },
            AnimationState::Waving => {
                // Wave animation
                let wave = (time.elapsed_seconds() * 5.0).sin() * 0.2;
                transform.rotation *= Quat::from_rotation_z(wave);
            },
            _ => {}
        }
    }
}

// Add this system for camera box positioning
fn camera_box_system(
    mut query: Query<(&mut Transform, &mut Player)>,
    camera_box: Res<CameraBox>,
    animation_settings: Res<AnimationSettings>,
    time: Res<Time>,
) {
    for (mut transform, _) in query.iter_mut() {
        // Ensure model stays within camera box bounds
        let target_pos = Vec3::new(
            transform.translation.x.clamp(
                camera_box.position.x - camera_box.dimensions.x / 2.0,
                camera_box.position.x + camera_box.dimensions.x / 2.0
            ),
            transform.translation.y.clamp(
                camera_box.position.y - camera_box.dimensions.y / 2.0,
                camera_box.position.y + camera_box.dimensions.y / 2.0
            ),
            transform.translation.z.clamp(
                camera_box.position.z - camera_box.dimensions.z / 2.0,
                camera_box.position.z + camera_box.dimensions.z / 2.0
            ),
        );

        // Smooth movement to target position
        transform.translation = transform.translation.lerp(
            target_pos,
            animation_settings.movement_smoothness * time.delta_seconds()
        );
    }
}

// Add this system for dynamic model scaling
fn model_scale_system(
    mut query: Query<(&mut Transform, &Player)>,
    animation_settings: Res<AnimationSettings>,
    time: Res<Time>,
) {
    for (mut transform, _) in query.iter_mut() {
        let base_scale = animation_settings.scale_range.0;
        let scale_range = animation_settings.scale_range.1 - animation_settings.scale_range.0;
        let scale_factor = (time.elapsed_seconds() * 0.5).sin() * 0.5 + 0.5;
        let target_scale = base_scale + scale_range * scale_factor;
        
        transform.scale = transform.scale.lerp(
            Vec3::splat(target_scale),
            animation_settings.movement_smoothness * time.delta_seconds()
        );
    }
}

// Add the neural behavior system
fn neural_behavior_system(
    time: Res<Time>,
    mut query: Query<(&mut ModelAnimation, &mut Transform)>,
    mut behavior_context: ResMut<BehaviorContext>,
    animation_settings: Res<AnimationSettings>,
) {
    // Update emotional state based on recent interactions
    update_emotional_state(&mut behavior_context, time.delta_seconds());

    for (mut animation, mut transform) in query.iter_mut() {
        // Generate behavior based on emotional state and context
        let behavior = generate_contextual_behavior(&behavior_context);
        
        // Apply the generated behavior
        apply_emotional_behavior(
            &mut transform,
            &mut animation,
            &behavior,
            &behavior_context.emotional_state,
            time.delta_seconds(),
        );
    }
}

fn update_emotional_state(context: &mut BehaviorContext, delta_time: f32) {
    let mut emotional_state = &mut context.emotional_state;
    
    // Natural emotional decay
    emotional_state.happiness = lerp(emotional_state.happiness, 0.5, 0.1 * delta_time);
    emotional_state.energy = lerp(emotional_state.energy, 0.5, 0.05 * delta_time);
    emotional_state.engagement = lerp(emotional_state.engagement, 0.3, 0.15 * delta_time);
    
    // Process recent interactions
    for interaction in &context.recent_interactions {
        apply_interaction_impact(emotional_state, interaction);
    }
    
    // Clean up old interactions
    context.recent_interactions.retain(|i| 
        Utc::now().signed_duration_since(i.timestamp) < chrono::Duration::seconds(30)
    );
}

fn generate_contextual_behavior(context: &BehaviorContext) -> Vec<AnimationState> {
    let mut behaviors = Vec::new();
    let emotional_state = &context.emotional_state;
    let personality = &context.personality_traits;
    
    // Generate behaviors based on emotional state and personality
    if emotional_state.happiness > 0.7 {
        behaviors.push(AnimationState::Waving);
    }
    
    if emotional_state.energy > 0.6 {
        behaviors.push(AnimationState::Walking);
    }
    
    if emotional_state.engagement > 0.5 {
        behaviors.push(AnimationState::Talking);
    }
    
    // Add personality-influenced behaviors
    if personality.extraversion > 0.6 && rand::random::<f32>() < 0.3 {
        behaviors.push(AnimationState::Reacting);
    }
    
    behaviors
}

fn apply_emotional_behavior(
    transform: &mut Transform,
    animation: &mut ModelAnimation,
    behaviors: &[AnimationState],
    emotional_state: &EmotionalState,
    delta_time: f32,
) {
    // Base movement influenced by emotional state
    let movement_scale = emotional_state.energy * 0.5 + 0.5;
    let rotation_scale = emotional_state.engagement * 0.3 + 0.7;
    
    // Apply emotional influences to movement
    for behavior in behaviors {
        match behavior {
            AnimationState::Talking => {
                let talk_intensity = emotional_state.engagement * 0.1;
                let head_motion = (time_elapsed() * 8.0).sin() * talk_intensity;
                transform.rotation *= Quat::from_rotation_y(head_motion);
            },
            AnimationState::Waving => {
                let wave_intensity = emotional_state.happiness * 0.2;
                let wave = (time_elapsed() * 5.0).sin() * wave_intensity;
                transform.rotation *= Quat::from_rotation_z(wave);
            },
            AnimationState::Walking => {
                let walk_speed = emotional_state.energy * movement_scale;
                transform.translation.x += walk_speed * delta_time;
            },
            AnimationState::Reacting => {
                let reaction_intensity = emotional_state.engagement * 0.15;
                transform.scale = Vec3::splat(1.0 + reaction_intensity * (time_elapsed() * 3.0).sin());
            },
            _ => {}
        }
    }
}

// Add this system for advanced cognitive processing
fn cognitive_processing_system(
    mut behavior_context: ResMut<BehaviorContext>,
    mut cognitive_capabilities: ResMut<CognitiveCapabilities>,
    mut cyber_expertise: ResMut<CyberSecurityExpertise>,
    time: Res<Time>,
) {
    // Continuous learning and adaptation
    update_knowledge_base(&mut cognitive_capabilities, &mut cyber_expertise, time.delta_seconds());
    
    // Advanced threat analysis and pattern recognition
    analyze_security_patterns(&mut cyber_expertise, &behavior_context);
    
    // Quantum-resistant security protocol generation
    generate_security_protocols(&mut cyber_expertise);
}

fn update_knowledge_base(
    cognitive: &mut CognitiveCapabilities,
    cyber: &mut CyberSecurityExpertise,
    delta_time: f32,
) {
    // Simulate continuous learning
    let learning_factor = cognitive.learning_rate * delta_time;
    
    for (domain, expertise_level) in cyber.domains.iter_mut() {
        match domain {
            SecurityDomain::AISecuritySystems => {
                *expertise_level += learning_factor * 1.5; // Accelerated AI security learning
            },
            SecurityDomain::QuantumCryptography => {
                *expertise_level += learning_factor * 1.3; // Advanced quantum security
            },
            _ => {
                *expertise_level += learning_factor;
            }
        }
        *expertise_level = expertise_level.min(1.0); // Cap expertise at 100%
    }
}

fn analyze_security_patterns(
    cyber: &mut CyberSecurityExpertise,
    context: &BehaviorContext,
) {
    // Advanced pattern recognition for threat detection
    cyber.threat_analysis_capability *= 1.001; // Continuous improvement
    
    // Zero-day vulnerability detection
    if cyber.zero_day_detection > 0.9 {
        // Implement advanced vulnerability scanning
        scan_for_vulnerabilities(cyber);
    }
}

fn generate_security_protocols(cyber: &mut CyberSecurityExpertise) {
    if cyber.domains.get(&SecurityDomain::QuantumCryptography).unwrap_or(&0.0) > 0.8 {
        // Generate quantum-resistant encryption protocols
        implement_quantum_resistant_protocols();
    }
}

// Add these security-related helper functions
fn scan_for_vulnerabilities(cyber: &CyberSecurityExpertise) {
    // Implement advanced vulnerability scanning logic
    let scan_effectiveness = cyber.threat_analysis_capability * cyber.zero_day_detection;
    if scan_effectiveness > 0.85 {
        // Perform deep security analysis
        implement_security_measures();
    }
}

fn implement_quantum_resistant_protocols() {
    // Implement quantum-resistant security protocols
    // This would be where you implement actual quantum-resistant algorithms
}

fn implement_security_measures() {
    // Implement advanced security measures
    // This would be where you implement actual security fixes
}

// Update the setup function to include advanced capabilities
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera setup
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn the model with all components
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("model.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player,
        AutoMove {
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            direction: Vec3::new(1.0, 0.0, 0.0),
        },
        ModelAnimation {
            idle_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            gesture_timer: Timer::from_seconds(5.0, TimerMode::Repeating),
            current_state: AnimationState::Idle,
        },
    ));

    // Add resources
    commands.insert_resource(MovementSettings {
        speed: 5.0,
        change_direction_interval: 3.0,
        movement_range: 10.0,
    });

    commands.insert_resource(AnimationSettings {
        idle_duration: 1.0,
        gesture_frequency: 5.0,
        scale_range: (0.8, 1.2),
        movement_smoothness: 5.0,
    });

    commands.insert_resource(CameraBox {
        position: Vec3::new(0.0, 1.0, 0.0),  // Center of camera box
        dimensions: Vec3::new(4.0, 3.0, 2.0), // Width, height, depth
    });

    // Add neural network resources
    commands.insert_resource(BehaviorContext {
        recent_interactions: Vec::new(),
        emotional_state: EmotionalState {
            happiness: 0.5,
            energy: 0.7,
            engagement: 0.6,
            confidence: 0.5,
        },
        attention_focus: AttentionPoint {
            target: "viewer".to_string(),
            intensity: 0.5,
            duration: std::time::Duration::from_secs(5),
        },
        personality_traits: PersonalityTraits {
            extraversion: 0.6,
            adaptability: 0.7,
            empathy: 0.8,
            creativity: 0.6,
        },
    });

    // Initialize advanced cognitive capabilities
    commands.insert_resource(CognitiveCapabilities {
        iq_level: 175.0,              // Exceptional IQ level
        learning_rate: 0.95,          // Very high learning rate
        pattern_recognition: 0.98,     // Superior pattern recognition
        problem_solving: 0.96,         // Advanced problem-solving
        memory_capacity: 0.99,         // Exceptional memory
        processing_speed: 0.97,        // Fast processing
    });

    // Initialize cybersecurity expertise
    let mut security_domains = HashMap::new();
    security_domains.insert(SecurityDomain::NetworkSecurity, 0.95);
    security_domains.insert(SecurityDomain::ApplicationSecurity, 0.93);
    security_domains.insert(SecurityDomain::CloudSecurity, 0.94);
    security_domains.insert(SecurityDomain::BlockchainSecurity, 0.92);
    security_domains.insert(SecurityDomain::AISecuritySystems, 0.96);
    security_domains.insert(SecurityDomain::QuantumCryptography, 0.91);
    security_domains.insert(SecurityDomain::ZeroTrustArchitecture, 0.94);
    security_domains.insert(SecurityDomain::AdvancedPenetrationTesting, 0.95);
    security_domains.insert(SecurityDomain::ThreatIntelligence, 0.93);
    security_domains.insert(SecurityDomain::IncidentResponse, 0.94);

    commands.insert_resource(CyberSecurityExpertise {
        domains: security_domains,
        certifications: vec![
            "CISSP".to_string(),
            "CISM".to_string(),
            "OSCP".to_string(),
            "CEH".to_string(),
            "CompTIA Security+".to_string(),
            "AWS Security Specialty".to_string(),
            "Google Cloud Security".to_string(),
            "Quantum Security Expert".to_string(),
        ],
        threat_analysis_capability: 0.95,
        incident_response_skill: 0.94,
        zero_day_detection: 0.92,
    });
}

// Helper function for linear interpolation
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t.clamp(0.0, 1.0)
}

// Helper function for getting elapsed time
fn time_elapsed() -> f32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f32()
}

// Update your App setup in main
fn main() {
    App::new()
        // ... existing systems ...
        .add_systems(Update, (
            auto_movement_system,
            model_animation_system,
            camera_box_system,
            model_scale_system,
            neural_behavior_system,
            cognitive_processing_system, // Add the new cognitive system
        ))
        .run();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CUDA available: {}", tch::Cuda::is_available());
    println!("CUDA device count: {}", tch::Cuda::device_count());

    // Initialize Twitch API
    let api = TwitchAPI::new().await?;

    // Example stream info
    let stream_info = StreamInfo {
        streamer_name: "YourName".to_string(),
        streamer_id: env::var("TWITCH_CHANNEL_ID")?,
        current_activity: "playing Minecraft and building an epic castle".to_string(),
        stream_category: "variety".to_string(),
        stream_tags: vec![
            "loves creating cozy vibes".to_string(),
            "enjoys chatting with community".to_string(),
            "focuses on positive gaming experiences".to_string(),
        ],
        auto_follow_raiders: true,
    };

    // Example: Handle incoming raid
    let raider = StreamerInfo {
        username: "raider_username".to_string(),
        display_name: "CoolRaider".to_string(),
        stream_title: "Speed running Mario 64".to_string(),
        category: "Super Mario 64".to_string(),
        uptime_minutes: 120,
        is_live: true,
    };

    handle_raid(&api, &stream_info, &raider).await?;

    // Add personality profile
    let personality = StreamerPersonality {
        interests: vec![
            "creating cozy gaming experiences".to_string(),
            "building awesome communities".to_string(),
            "trying new indie games".to_string(),
        ],
        streaming_style: "focuses on positive vibes and community interaction".to_string(),
        favorite_games: vec![
            "Minecraft".to_string(),
            "Stardew Valley".to_string(),
            "Animal Crossing".to_string(),
        ],
        recent_achievements: vec![
            "just finished our first community build project".to_string(),
            "reached affiliate status".to_string(),
            "had our biggest raid train ever last week".to_string(),
        ],
        community_values: vec![
            "positivity".to_string(),
            "inclusivity".to_string(),
            "creativity".to_string(),
            "supporting each other".to_string(),
        ],
    };

    // Update the select_and_raid call to include personality
    select_and_raid(&api, &stream_info, &personality).await?;

    Ok(())
}

// Add these new types for explanation handling
#[derive(Debug, Clone)]
struct ExplanationStyle {
    audience_level: AudienceLevel,
    use_analogies: bool,
    include_examples: bool,
    tone: ExplanationTone,
}

#[derive(Debug, Clone)]
enum AudienceLevel {
    Child,
    Senior,
    NonTechnical,
    Technical,
    Expert,
}

#[derive(Debug, Clone)]
enum ExplanationTone {
    Friendly,
    Educational,
    Encouraging,
    Professional,
}

// Add this function to generate friendly security explanations
fn generate_friendly_security_explanation(
    topic: &SecurityDomain,
    audience: AudienceLevel,
    cyber_expertise: &CyberSecurityExpertise,
) -> String {
    let base_explanation = match topic {
        SecurityDomain::NetworkSecurity => {
            match audience {
                AudienceLevel::Child | AudienceLevel::Senior => {
                    "Think of network security like having a special guard for your house. \
                    Just like how we lock our doors and windows to keep safe, \
                    network security helps protect our computers and phones from bad people \
                    who might try to peek at our private things or cause trouble. \
                    We use special digital locks and alarm systems to keep everything safe! 🏠🔒"
                },
                AudienceLevel::NonTechnical => {
                    "Network security is like having a really smart security system for the internet. \
                    It protects all the information traveling between computers, kind of like how \
                    a mail carrier makes sure your packages arrive safely without being opened \
                    by anyone else. We use special tools to check that only the right people \
                    can access certain things online. 📨✨"
                },
                _ => {
                    "Network security encompasses the practices and policies designed to protect \
                    network infrastructure and data transmission. We implement multiple layers of \
                    defense including firewalls, encryption, and access controls. 🛡️"
                }
            }
        },
        SecurityDomain::QuantumCryptography => {
            match audience {
                AudienceLevel::Child | AudienceLevel::Senior => {
                    "Imagine you have a magical box that can only be opened by you and your friend. \
                    If anyone else tries to peek inside, the box immediately changes what's inside! \
                    That's kind of like quantum cryptography - it's a super special way to send \
                    secret messages that nobody else can read. Even if they try to look, \
                    they can't see the real message! ✨📦"
                },
                AudienceLevel::NonTechnical => {
                    "Quantum cryptography is like having an unbreakable secret code that uses \
                    the special rules of very tiny things (quantum physics). If anyone tries to \
                    intercept the message, it automatically scrambles itself - kind of like \
                    invisible ink that disappears if someone unauthorized tries to read it! 🔐"
                },
                _ => {
                    "Quantum cryptography leverages quantum mechanical properties to create \
                    theoretically unbreakable encryption. It uses quantum states of particles \
                    to detect any unauthorized observation of the data. 🌟"
                }
            }
        },
        SecurityDomain::AISecuritySystems => {
            match audience {
                AudienceLevel::Child | AudienceLevel::Senior => {
                    "Imagine having a super-smart helper that watches over your computer like \
                    a friendly guard dog! It learns what normal activities look like and can \
                    spot when something unusual is happening. If it sees anything suspicious, \
                    it lets us know right away - just like how a dog barks to warn us! 🐕💻"
                },
                AudienceLevel::NonTechnical => {
                    "AI security systems are like having a very intelligent security guard that \
                    never gets tired and can watch millions of things at once. It learns from \
                    patterns to spot unusual behavior and can respond much faster than humans. \
                    Think of it as a digital immune system for your computer! 🤖"
                },
                _ => {
                    "AI security systems utilize machine learning algorithms to detect and respond \
                    to threats in real-time. They can identify patterns, anomalies, and potential \
                    security breaches by analyzing vast amounts of data. 🎯"
                }
            }
        },
        // Add more domains as needed...
    }.to_string();

    // Add relevant emojis and friendly touches based on audience
    add_friendly_touches(base_explanation, audience)
}

// Helper function to add friendly elements to explanations
fn add_friendly_touches(mut explanation: String, audience: AudienceLevel) -> String {
    match audience {
        AudienceLevel::Child => {
            explanation.push_str("\n\nDoes that help explain it? Feel free to ask more questions! 😊");
        },
        AudienceLevel::Senior => {
            explanation.push_str("\n\nI hope that makes sense! Let me know if you'd like me to explain anything else. 💝");
        },
        AudienceLevel::NonTechnical => {
            explanation.push_str("\n\nI can explain more about any part that interests you! 💫");
        },
        _ => {
            explanation.push_str("\n\nWould you like to explore any specific aspect in more detail? 🔍");
        }
    }
    explanation
}

// Add this to the cognitive processing system
fn handle_security_question(
    topic: SecurityDomain,
    audience: AudienceLevel,
    cyber_expertise: &CyberSecurityExpertise,
    emotional_state: &EmotionalState,
) -> String {
    let base_explanation = generate_friendly_security_explanation(&topic, audience, cyber_expertise);
    
    // Adjust explanation based on emotional state
    let empathy_level = emotional_state.engagement * 0.7 + emotional_state.happiness * 0.3;
    
    if empathy_level > 0.8 {
        // Add extra encouraging elements for high empathy
        format!("{}\n\nYou're asking great questions! Security can seem complicated, but we can break it down together! 💫", base_explanation)
    } else {
        base_explanation
    }
}
