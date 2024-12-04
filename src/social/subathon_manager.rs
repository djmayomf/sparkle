use chrono::{DateTime, Utc};
use crate::moderation::ContentFilter;
use crate::ai::personality_core::PersonalityCore;

#[derive(Debug)]
pub struct SubathonManager {
    tweet_queue: TweetQueue,
    content_filter: ContentFilter,
    personality: Arc<RwLock<PersonalityCore>>,
    media_scheduler: MediaScheduler,
    sub_goals: SubGoalTracker,
}

#[derive(Debug)]
struct TweetQueue {
    pending_tweets: VecDeque<SubathonTweet>,
    approved_tweets: Vec<SubathonTweet>,
    rejected_tweets: Vec<RejectedTweet>,
}

#[derive(Debug)]
struct SubathonTweet {
    content: String,
    suggested_by: String,
    sub_goal_id: Option<u32>,
    votes: i32,
    submitted_at: DateTime<Utc>,
    moderation_status: ModerationStatus,
}

#[derive(Debug)]
struct MediaScheduler {
    post_stream_queue: Vec<PostStreamContent>,
    pre_stream_schedule: Vec<StreamPromotion>,
    optimal_timing: PostTimingOptimizer,
}

impl SubathonManager {
    pub async fn handle_tweet_suggestion(&mut self, suggestion: &str, user: &str) -> Result<TweetResponse> {
        // First pass moderation
        if !self.content_filter.is_appropriate(suggestion) {
            return Ok(TweetResponse::Rejected(
                "Tweet contains inappropriate content".to_string()
            ));
        }

        // Check if user can suggest tweets
        if !self.can_suggest_tweets(user).await? {
            return Ok(TweetResponse::NotAllowed(
                "Tweet suggestions are part of sub goals".to_string()
            ));
        }

        // Process and queue tweet
        let tweet = SubathonTweet {
            content: suggestion.to_string(),
            suggested_by: user.to_string(),
            sub_goal_id: self.sub_goals.current_goal_id(),
            votes: 0,
            submitted_at: Utc::now(),
            moderation_status: ModerationStatus::Pending,
        };

        self.tweet_queue.add_tweet(tweet).await?;
        
        Ok(TweetResponse::Accepted)
    }

    pub async fn process_media_post(&mut self, media: MediaContent, post_type: PostType) -> Result<()> {
        match post_type {
            PostType::PostStream => {
                let optimized = self.optimize_post_stream_content(media).await?;
                self.media_scheduler.queue_post_stream(optimized).await?;
            },
            PostType::StreamPromo => {
                let promo = self.create_stream_promo(media).await?;
                self.media_scheduler.schedule_pre_stream(promo).await?;
            }
        }

        Ok(())
    }

    async fn optimize_post_stream_content(&self, media: MediaContent) -> Result<PostStreamContent> {
        // Extract stream highlights
        let highlights = self.extract_stream_highlights(&media).await?;
        
        // Generate engaging captions
        let caption = self.generate_post_stream_caption(&highlights).await?;
        
        // Optimize media format
        let optimized_media = self.optimize_media_format(media, MediaPlatform::Instagram).await?;

        Ok(PostStreamContent {
            media: optimized_media,
            caption,
            highlights,
            posting_time: self.calculate_optimal_posting_time().await?,
        })
    }

    async fn create_stream_promo(&self, media: MediaContent) -> Result<StreamPromotion> {
        // Get stream schedule
        let schedule = self.get_stream_schedule().await?;
        
        // Generate hype content
        let promo_content = self.generate_stream_promo(schedule).await?;
        
        // Optimize for platforms
        let (twitter_media, instagram_media) = self.optimize_promo_media(media).await?;

        Ok(StreamPromotion {
            twitter: TwitterPromo {
                media: twitter_media,
                text: promo_content.twitter,
                schedule: schedule.clone(),
            },
            instagram: InstagramPromo {
                media: instagram_media,
                caption: promo_content.instagram,
                schedule,
            },
        })
    }
}

impl TweetQueue {
    pub async fn add_tweet(&mut self, tweet: SubathonTweet) -> Result<()> {
        // Additional safety checks
        if self.is_duplicate(&tweet) {
            return Err(Error::DuplicateTweet);
        }

        // Queue tweet for community voting
        self.pending_tweets.push_back(tweet);
        
        // Clean up old pending tweets
        self.cleanup_old_pending().await;
        
        Ok(())
    }

    pub async fn process_votes(&mut self) -> Result<Vec<SubathonTweet>> {
        let mut approved = Vec::new();
        
        // Process tweets with enough votes
        while let Some(tweet) = self.pending_tweets.pop_front() {
            if tweet.votes >= self.get_required_votes() {
                // Final moderation check
                if self.passes_final_moderation(&tweet).await? {
                    approved.push(tweet);
                }
            } else {
                // Put back if not enough votes
                self.pending_tweets.push_back(tweet);
            }
        }

        Ok(approved)
    }
}

impl MediaScheduler {
    async fn queue_post_stream(&mut self, content: PostStreamContent) -> Result<()> {
        // Optimize posting time
        let optimal_time = self.optimal_timing
            .calculate_post_stream_time(&content)
            .await?;

        // Schedule post
        self.post_stream_queue.push(PostStreamContent {
            posting_time: optimal_time,
            ..content
        });

        Ok(())
    }

    async fn schedule_pre_stream(&mut self, promo: StreamPromotion) -> Result<()> {
        // Calculate optimal pre-stream posting schedule
        let schedule = self.optimal_timing
            .calculate_promo_schedule(&promo)
            .await?;

        // Schedule posts across platforms
        for timing in schedule {
            self.pre_stream_schedule.push(StreamPromotion {
                posting_time: timing,
                ..promo.clone()
            });
        }

        Ok(())
    }
} 