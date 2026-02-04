use camb_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CAMB_API_KEY")
        .expect("CAMB_API_KEY environment variable not set");

    let client = APIClient::new(ClientConfig {
        api_key: Some(api_key),
        ..ClientConfig::default()
    })?;

    println!("Generating new voice...");

    let result = client.text_to_voice.create_text_to_voice(&CreateTextToVoiceRequestPayload {
        text: "A calm and soothing voice for meditation apps.".to_string(),
        voice_description: "Soft, slow-paced male voice with a deep resonance.".to_string(),
        project_name: None,
        project_description: None,
        folder_id: None,
    }, None).await?;

    println!("Voice Generation Task Started. Task ID: {:?}", result.task_id);

    Ok(())
}
