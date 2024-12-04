use crate::model::tracking::BodyTrackingSystem;
use crate::discord::webhook::DiscordWebhook;
use chrono::{DateTime, Utc, Duration};

pub struct StreamAnnouncer {
    twitter_client: TwitterClient,
    discord_webhook: DiscordWebhook,
    body_tracking: Arc<RwLock<BodyTrackingSystem>>,
    schedule_manager: ScheduleManager,
    subathon_state: SubathonState,
}

#[derive(Debug)]
struct SubathonState {
    is_active: bool,
    dance_mode_enabled: bool,
    tracking_status: TrackingStatus,
    movement_restrictions: Vec<MovementRestriction>,
}

#[derive(Debug)]
struct TrackingStatus {
    full_body_available: bool,
    tracking_quality: f32,
    active_tracking_points: Vec<TrackingPoint>,
    calibration_status: CalibrationStatus,
}

impl StreamAnnouncer {
    pub async fn announce_stream_schedule(&self, schedule: StreamSchedule) -> Result<()> {
        let announcement = self.create_schedule_announcement(&schedule).await?;
        
        // Post to Twitter
        self.twitter_client.tweet(TwitterPost {
            content: announcement.twitter_format,
            media: Some(schedule.thumbnail),
            schedule: Some(schedule.start_time),
        }).await?;

        // Post to Discord
        self.discord_webhook.send_announcement(DiscordAnnouncement {
            content: announcement.discord_format,
            embed: self.create_schedule_embed(&schedule),
            ping_role: Some(schedule.notification_role),
        }).await?;

        Ok(())
    }

    pub async fn announce_stream_start(&self) -> Result<()> {
        let stream_info = self.get_current_stream_info().await?;
        
        // Create live announcements
        let twitter_post = format!(
            "🔴 LIVE NOW!\n\n{}\n\n{}\n\nCome hang out! 💫",
            stream_info.title,
            stream_info.category
        );

        let discord_embed = self.create_live_embed(&stream_info);

        // Post announcements
        tokio::try_join!(
            self.twitter_client.tweet(TwitterPost {
                content: twitter_post,
                media: Some(stream_info.thumbnail),
                schedule: None,
            }),
            self.discord_webhook.send_live_notification(LiveNotification {
                embed: discord_embed,
                ping_role: stream_info.notification_role,
            })
        )?;

        Ok(())
    }

    pub async fn handle_subathon_dance_request(&mut self, dance_type: DanceType) -> Result<DanceResponse> {
        // Check if subathon is active
        if !self.subathon_state.is_active {
            return Ok(DanceResponse::SubathonInactive);
        }

        // Verify body tracking status
        let tracking_status = self.body_tracking.read().await.get_status();
        
        if !tracking_status.full_body_available {
            return Ok(DanceResponse::TrackingUnavailable(
                "Full body tracking is required for dance mode".to_string()
            ));
        }

        // Verify tracking quality
        if tracking_status.tracking_quality < 0.8 {
            return Ok(DanceResponse::TrackingQualityLow);
        }

        // Enable dance mode with verified tracking
        self.subathon_state.dance_mode_enabled = true;
        self.body_tracking.write().await.enable_dance_mode(dance_type)?;

        Ok(DanceResponse::Enabled)
    }

    async fn create_schedule_announcement(&self, schedule: &StreamSchedule) -> Result<Announcement> {
        let start_time = schedule.start_time.format("%I:%M %p %Z").to_string();
        let date = schedule.start_time.format("%A, %B %d").to_string();

        let twitter_format = format!(
            "📅 Stream Schedule Update!\n\n\
            Going live on {} at {}!\n\n\
            We'll be {}!\n\n\
            Set your reminders! 🔔",
            date, start_time, schedule.activity
        );

        let discord_format = format!(
            "**Stream Schedule Update!**\n\n\
            📅 Date: {}\n\
            ⏰ Time: {}\n\
            🎮 Activity: {}\n\n\
            Don't miss out! Hit the 🔔 to get notified when we go live!",
            date, start_time, schedule.activity
        );

        Ok(Announcement {
            twitter_format,
            discord_format,
        })
    }

    fn create_live_embed(&self, stream_info: &StreamInfo) -> DiscordEmbed {
        DiscordEmbed::new()
            .title("🔴 NOW LIVE!")
            .description(&stream_info.title)
            .field("Category", &stream_info.category, true)
            .field("Started", "Just now!", true)
            .thumbnail(stream_info.thumbnail.clone())
            .color(0xFF0000) // Red color for live status
            .build()
    }
}

impl BodyTrackingSystem {
    pub fn get_status(&self) -> TrackingStatus {
        TrackingStatus {
            full_body_available: self.verify_full_body_tracking(),
            tracking_quality: self.calculate_tracking_quality(),
            active_tracking_points: self.get_active_tracking_points(),
            calibration_status: self.get_calibration_status(),
        }
    }

    fn verify_full_body_tracking(&self) -> bool {
        // Verify all required tracking points are active
        let required_points = vec![
            TrackingPoint::Head,
            TrackingPoint::Torso,
            TrackingPoint::LeftArm,
            TrackingPoint::RightArm,
            TrackingPoint::LeftLeg,
            TrackingPoint::RightLeg,
            TrackingPoint::LeftFoot,
            TrackingPoint::RightFoot,
        ];

        required_points.iter().all(|point| self.is_point_tracked(point))
    }

    fn calculate_tracking_quality(&self) -> f32 {
        let tracking_data = self.get_tracking_data();
        
        // Calculate average tracking quality across all points
        let total_quality: f32 = tracking_data.iter()
            .map(|point| point.tracking_confidence)
            .sum();

        total_quality / tracking_data.len() as f32
    }

    pub async fn enable_dance_mode(&mut self, dance_type: DanceType) -> Result<()> {
        // Set up tracking for dance movements
        self.set_tracking_mode(TrackingMode::HighPrecision);
        
        // Configure movement bounds for dance
        self.set_movement_bounds(dance_type.get_movement_bounds());
        
        // Enable smooth motion interpolation
        self.enable_motion_smoothing(SmoothingLevel::High);
        
        // Start movement validation
        self.start_movement_validation();

        Ok(())
    }
} 