use camb_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CAMB_API_KEY")
        .expect("CAMB_API_KEY environment variable not set");

    let client = APIClient::new(ClientConfig {
        api_key: Some(api_key),
        ..ClientConfig::default()
    })?;

    println!("Generating sound effect...");

    let result = client.text_to_audio.create_text_to_audio(&CreateTextToAudioRequestPayload {
        prompt: "A futuristic laser blast followed by an echo.".to_string(),
        project_name: None,
        project_description: None,
        folder_id: None,
        duration: None,
        audio_type: None,
        run_id: None,
    }, None).await?;

    println!("Sound Generation Started. Task ID: {:?}", result.task_id);

    Ok(())
}
