use camb_api::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("CAMB_API_KEY")
        .expect("CAMB_API_KEY environment variable not set");

    let client = APIClient::new(ClientConfig {
        api_key: Some(api_key),
        ..ClientConfig::default()
    })?;

    println!("Starting Dubbing Task...");

    // Using the end_to_end_dubbing method and manual struct init
    let result = client.dub.end_to_end_dubbing(&EndToEndDubbingRequestPayload {
        video_url: "https://example.com/video.mp4".to_string(),
        source_language: Languages::EN_US,
        target_languages: Some(Some(vec![Languages::FR_FR, Languages::ES_ES])),
        project_name: None,
        project_description: None,
        folder_id: None,
        target_language: None,
        selected_audio_tracks: None,
        add_output_as_an_audio_track: None,
        chosen_dictionaries: None,
        ai_optimization: None,
        run_id: None,
    }, None).await?;

    println!("Dubbing Task Created. Task ID: {:?}", result.task_id);

    Ok(())
}
