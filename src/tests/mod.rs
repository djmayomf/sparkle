use super::*;
use tokio_test::block_on;
use mock_server::MockServer;

#[tokio::test]
async fn test_personality_generation() {
    let scraper = YouTubePersonalityScraper::new("test_playlist").unwrap();
    let response = scraper.get_unique_response("test", 0.5).await.unwrap();
    assert!(!response.is_empty());
}

#[tokio::test]
async fn test_error_handling() {
    let scraper = YouTubePersonalityScraper::new("").unwrap_err();
    assert!(matches!(scraper, AppError::PersonalityGeneration(_)));
}

#[tokio::test]
async fn test_cache_behavior() {
    let mut scraper = YouTubePersonalityScraper::new("test_playlist").unwrap();
    
    // Test cache insertion
    let response1 = scraper.get_unique_response("test", 0.5).await.unwrap();
    let response2 = scraper.get_unique_response("test", 0.5).await.unwrap();
    
    assert_eq!(response1, response2); // Should hit cache
} 