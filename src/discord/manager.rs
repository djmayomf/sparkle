use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use crate::ai::personality_core::PersonalityCore;
use crate::database::connection::DbPool;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct DiscordManager {
    client: Client,
    personality: Arc<RwLock<PersonalityCore>>,
    db: DbPool,
    server_config: ServerConfig,
    community_manager: CommunityManager,
    event_handler: CustomEventHandler,
}

#[derive(Debug, Clone)]
struct ServerConfig {
    welcome_channel_id: ChannelId,
    rules_channel_id: ChannelId,
    announcement_channel_id: ChannelId,
    roles: Vec<RoleConfig>,
    auto_mod_settings: AutoModSettings,
    custom_commands: HashMap<String, CustomCommand>,
}

#[derive(Debug, Clone)]
struct RoleConfig {
    name: String,
    color: u32,
    permissions: Permissions,
    auto_assign: bool,
    requirements: Vec<RoleRequirement>,
}

#[derive(Debug, Clone)]
struct AutoModSettings {
    spam_threshold: u32,
    word_filter: Vec<String>,
    link_policy: LinkPolicy,
    raid_protection: bool,
    verification_level: VerificationLevel,
}

#[derive(Debug, Clone)]
struct ServerChannels {
    // Information Category
    welcome: ChannelId,
    rules: ChannelId,
    announcements: ChannelId,
    
    // Community Category
    general: ChannelId,
    introductions: ChannelId,
    fan_art: ChannelId,
    memes: ChannelId,
    
    // Gaming Category
    gaming_general: ChannelId,
    clips_highlights: ChannelId,
    game_discussion: ChannelId,
    
    // Tech & Security Category
    tech_help: ChannelId,
    coding_corner: ChannelId,
    security_talks: ChannelId,
    
    // Anime & Culture Category
    anime_discussion: ChannelId,
    tokusatsu_corner: ChannelId,
    fan_creations: ChannelId,
    
    // Events & Activities
    events: ChannelId,
    voice_channels: Vec<ChannelId>,
}

impl DiscordManager {
    pub async fn new(token: &str, db: DbPool, personality: Arc<RwLock<PersonalityCore>>) -> Result<Self> {
        let framework = StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .group(&GENERAL_GROUP);

        let intents = GatewayIntents::non_privileged() 
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MEMBERS;

        let client = Client::builder(token, intents)
            .framework(framework)
            .event_handler(CustomEventHandler::new())
            .await?;

        Ok(Self {
            client,
            personality,
            db,
            server_config: ServerConfig::default(),
            community_manager: CommunityManager::new(),
            event_handler: CustomEventHandler::new(),
        })
    }

    pub async fn setup_new_server(&mut self, guild_id: GuildId) -> Result<()> {
        // Create basic channel structure
        let channels = self.create_base_channels(guild_id).await?;
        
        // Set up roles and permissions
        let roles = self.setup_roles(guild_id).await?;
        
        // Configure welcome message and rules
        self.setup_welcome_system(channels.welcome, &roles).await?;
        
        // Set up automod and security
        self.configure_automod(guild_id).await?;
        
        // Initialize community features
        self.setup_community_features(guild_id, &channels).await?;

        Ok(())
    }

    async fn create_base_channels(&self, guild_id: GuildId) -> Result<ServerChannels> {
        let guild = guild_id.to_partial_guild(&self.client).await?;

        // Create categories
        let info_category = self.create_category(&guild, "📢 Information").await?;
        let community_category = self.create_category(&guild, "🌟 Community").await?;
        let gaming_category = self.create_category(&guild, "🎮 Gaming").await?;
        let tech_category = self.create_category(&guild, "💻 Tech & Security").await?;
        let anime_category = self.create_category(&guild, "🎭 Anime & Culture").await?;
        let events_category = self.create_category(&guild, "📅 Events & Activities").await?;

        // Information Channels
        let welcome = self.create_text_channel(&guild, "👋-welcome", &info_category, 
            "Welcome to our community! Get started here.").await?;
        let rules = self.create_text_channel(&guild, "📜-rules", &info_category,
            "Server rules and guidelines").await?;
        let announcements = self.create_text_channel(&guild, "📢-announcements", &info_category,
            "Important server announcements").await?;

        // Community Channels
        let general = self.create_text_channel(&guild, "💬-general", &community_category,
            "General community chat").await?;
        let introductions = self.create_text_channel(&guild, "👤-introductions", &community_category,
            "Introduce yourself to the community!").await?;
        let fan_art = self.create_art_channel(&guild, &community_category).await?;
        let memes = self.create_text_channel(&guild, "😄-memes", &community_category,
            "Share your favorite memes").await?;

        // Gaming Channels
        let gaming_general = self.create_text_channel(&guild, "🎮-gaming", &gaming_category,
            "General gaming discussion").await?;
        let clips_highlights = self.create_text_channel(&guild, "🎥-clips", &gaming_category,
            "Share your best gaming moments").await?;
        let game_discussion = self.create_text_channel(&guild, "💭-game-discussion", &gaming_category,
            "In-depth game discussions").await?;

        // Tech & Security Channels
        let tech_help = self.create_text_channel(&guild, "🔧-tech-help", &tech_category,
            "Get help with technical issues").await?;
        let coding_corner = self.create_text_channel(&guild, "👩‍💻-coding", &tech_category,
            "Programming discussions and help").await?;
        let security_talks = self.create_text_channel(&guild, "🔒-security", &tech_category,
            "Cybersecurity discussions and tips").await?;

        // Anime & Culture Channels
        let anime_discussion = self.create_text_channel(&guild, "🎌-anime", &anime_category,
            "Discuss your favorite anime").await?;
        let tokusatsu_corner = self.create_text_channel(&guild, "⚡-tokusatsu", &anime_category,
            "All things tokusatsu").await?;
        let fan_creations = self.create_text_channel(&guild, "🎨-fan-creations", &anime_category,
            "Share your fan creations").await?;

        // Voice Channels
        let voice_channels = self.create_voice_channels(&guild, &events_category).await?;

        Ok(ServerChannels {
            welcome: welcome.id,
            rules: rules.id,
            announcements: announcements.id,
            general: general.id,
            introductions: introductions.id,
            fan_art: fan_art.id,
            memes: memes.id,
            gaming_general: gaming_general.id,
            clips_highlights: clips_highlights.id,
            game_discussion: game_discussion.id,
            tech_help: tech_help.id,
            coding_corner: coding_corner.id,
            security_talks: security_talks.id,
            anime_discussion: anime_discussion.id,
            tokusatsu_corner: tokusatsu_corner.id,
            fan_creations: fan_creations.id,
            events: events_category.id,
            voice_channels,
        })
    }

    async fn create_art_channel(&self, guild: &Guild, category: &GuildChannel) -> Result<GuildChannel> {
        let channel = guild.create_channel(&self.client, |c| {
            c.name("🎨-fan-art")
             .kind(ChannelType::Text)
             .category(category)
             .topic("Share your amazing fan art! Supported formats: PNG, JPG, GIF")
             .rate_limit_per_user(30) // 30 second slowmode
        }).await?;

        // Set up art channel specific permissions
        guild.edit_role(&self.client, &RoleId(guild.id.0), |r| {
            r.permissions(Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES | Permissions::ATTACH_FILES)
        }).await?;

        // Send art channel guidelines
        channel.send_message(&self.client, |m| {
            m.embed(|e| {
                e.title("🎨 Fan Art Channel Guidelines")
                 .description("\
                    Welcome to the Fan Art channel! Here are some guidelines:\n\
                    \n\
                    • Original art only - credit others if sharing with permission\n\
                    • Use appropriate file formats (PNG, JPG, GIF)\n\
                    • Keep art family-friendly\n\
                    • Include a brief description of your work\n\
                    • Use spoiler tags for content from recent episodes\n\
                    • Be supportive of other artists!\n\
                    \n\
                    React with ⭐ to show appreciation for others' work!")
                 .color(0xFF69B4)
            })
        }).await?;

        Ok(channel)
    }

    async fn create_voice_channels(&self, guild: &Guild, category: &GuildChannel) -> Result<Vec<ChannelId>> {
        let voice_channels = vec![
            ("🎮 Gaming Lounge", 0),
            ("🎵 Music & Chill", 0),
            ("💬 General Chat", 0),
            ("🎲 Game Night", 0),
            ("📺 Watch Party", 0),
        ];

        let mut channel_ids = Vec::new();

        for (name, user_limit) in voice_channels {
            let channel = guild.create_channel(&self.client, |c| {
                c.name(name)
                 .kind(ChannelType::Voice)
                 .category(category)
                 .user_limit(user_limit)
            }).await?;
            channel_ids.push(channel.id);
        }

        Ok(channel_ids)
    }

    async fn setup_roles(&self, guild_id: GuildId) -> Result<Vec<Role>> {
        let guild = guild_id.to_partial_guild(&self.client).await?;
        let mut roles = Vec::new();

        // Create admin role
        let admin = guild.create_role(&self.client, |r| {
            r.name("Admin")
             .colour(0xFF0000)
             .permissions(Permissions::ADMINISTRATOR)
             .hoist(true)
        }).await?;
        roles.push(admin);

        // Create moderator role
        let mod_role = guild.create_role(&self.client, |r| {
            r.name("Moderator")
             .colour(0x00FF00)
             .permissions(Permissions::MANAGE_MESSAGES | Permissions::KICK_MEMBERS)
             .hoist(true)
        }).await?;
        roles.push(mod_role);

        // Create member role
        let member = guild.create_role(&self.client, |r| {
            r.name("Member")
             .colour(0x0000FF)
             .permissions(Permissions::READ_MESSAGES | Permissions::SEND_MESSAGES)
        }).await?;
        roles.push(member);

        Ok(roles)
    }

    async fn setup_welcome_system(&self, channel_id: ChannelId, roles: &[Role]) -> Result<()> {
        let welcome_message = format!(
            "Welcome to our server! 🎉\n\
            \n\
            Please read our rules in <#{}> and react to this message to get the Member role.\n\
            \n\
            Some helpful commands to get started:\n\
            `!help` - Show available commands\n\
            `!roles` - View available roles\n\
            `!profile` - Set up your profile\n\
            \n\
            Enjoy your stay! 💫",
            self.server_config.rules_channel_id
        );

        let msg = channel_id.send_message(&self.client, |m| {
            m.content(welcome_message)
             .reactions(['✅'])
        }).await?;

        // Store message ID for reaction role system
        self.server_config.welcome_message_id = Some(msg.id);

        Ok(())
    }

    async fn configure_automod(&self, guild_id: GuildId) -> Result<()> {
        let guild = guild_id.to_partial_guild(&self.client).await?;

        // Set verification level
        guild.edit(&self.client, |g| {
            g.verification_level(VerificationLevel::Medium)
        }).await?;

        // Configure auto-mod settings
        self.server_config.auto_mod_settings = AutoModSettings {
            spam_threshold: 5,
            word_filter: vec!["spam".to_string(), "offensive".to_string()],
            link_policy: LinkPolicy::MembersOnly,
            raid_protection: true,
            verification_level: VerificationLevel::Medium,
        };

        Ok(())
    }

    async fn setup_community_features(&self, guild_id: GuildId, channels: &ServerChannels) -> Result<()> {
        // Set up leveling system
        self.community_manager.initialize_leveling_system(guild_id).await?;
        
        // Configure custom commands
        self.setup_custom_commands().await?;
        
        // Set up event scheduling
        self.community_manager.initialize_event_system(guild_id).await?;
        
        // Configure automated announcements
        self.setup_automated_announcements(channels.announcements).await?;

        Ok(())
    }

    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<()> {
        match interaction {
            Interaction::Command(command) => {
                self.handle_command(command).await?;
            },
            Interaction::Component(component) => {
                self.handle_component(component).await?;
            },
            _ => {}
        }
        Ok(())
    }
}

#[group]
#[commands(help, profile, roles)]
struct General;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, 
        "Available commands:\n\
        !help - Show this message\n\
        !profile - Set up your profile\n\
        !roles - View available roles"
    ).await?;
    Ok(())
}

struct CustomEventHandler;

impl EventHandler for CustomEventHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        
        // Process message with personality
        if let Err(e) = process_message(&ctx, &msg).await {
            println!("Error processing message: {:?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
} 