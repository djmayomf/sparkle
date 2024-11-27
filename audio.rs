// src/audio.rs

use reqwest::Client;
use serde_json::json;

pub async fn transcribe_audio(audio_file_uri: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = "YOUR_API_KEY"; // Replace with your actual API key

    let request_body = json!({
        "config": {
            "encoding": "LINEAR16",
            "sampleRateHertz": 16000,
            "languageCode": "en-US",
        },
        "audio": {
            "uri": audio_file_uri,
        }
    });

    let response = client.post(&format!("https://speech.googleapis.com/v1/speech:recognize?key={}", api_key))
        .json(&request_body)
        .send()
        .await?;

    let response_json: serde_json::Value = response.json().await?;
    println!("{:?}", response_json);

    Ok(())
}