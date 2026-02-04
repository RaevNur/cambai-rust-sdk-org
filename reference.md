# Reference
<details><summary><code>client.<a href="/src/client.rs">get_swagger_docs_docs_get</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.get_swagger_docs_docs_get(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_redoc_docs_redocs_get</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.get_redoc_docs_redocs_get(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.<a href="/src/client.rs">get_openapi_schema_openapi_json_get</a>() -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.get_openapi_schema_openapi_json_get(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## AudioSeparation
<details><summary><code>client.audio_separation.<a href="/src/api/resources/audio_separation/client.rs">create_audio_separation</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineCallResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .audio_separation
        .create_audio_separation(
            &CreateAudioSeparationRequest {
                media_file: b"test file content".to_vec(),
                project_name: None,
                project_description: None,
                folder_id: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audio_separation.<a href="/src/api/resources/audio_separation/client.rs">get_audio_separation_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .audio_separation
        .get_audio_separation_status(
            &"task_id".to_string(),
            &GetAudioSeparationStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audio_separation.<a href="/src/api/resources/audio_separation/client.rs">get_audio_separation_run_info</a>(run_id: Option&lt;i64&gt;) -> Result&lt;GetAudioSeparationResultOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .audio_separation
        .get_audio_separation_run_info(&Some(1), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.audio_separation.<a href="/src/api/resources/audio_separation/client.rs">get_audio_separation_runs_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, GetAudioSeparationResultOut&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .audio_separation
        .get_audio_separation_runs_results(
            &GetAudioSeparationRunsResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Dub
<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">end_to_end_dubbing</a>(request: EndToEndDubbingRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineCallResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .end_to_end_dubbing(
            &EndToEndDubbingRequestPayload {
                video_url: "video_url".to_string(),
                source_language: Languages(1),
                run_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                target_language: None,
                target_languages: None,
                selected_audio_tracks: None,
                add_output_as_an_audio_track: None,
                chosen_dictionaries: None,
                ai_optimization: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**video_url:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**source_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**target_language:** `Option<Option<Languages>>` 
    
</dd>
</dl>

<dl>
<dd>

**target_languages:** `Option<Option<Vec<Languages>>>` 
    
</dd>
</dl>

<dl>
<dd>

**selected_audio_tracks:** `Option<Option<Vec<i64>>>` 
    
</dd>
</dl>

<dl>
<dd>

**add_output_as_an_audio_track:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**chosen_dictionaries:** `Option<Option<Vec<i64>>>` 
    
</dd>
</dl>

<dl>
<dd>

**ai_optimization:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">get_end_to_end_dubbing_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .get_end_to_end_dubbing_status(
            &"task_id".to_string(),
            &GetEndToEndDubbingStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">get_dubbed_run_info</a>(run_id: Option&lt;i64&gt;) -> Result&lt;GetDubbedRunInfoDubResultRunIDGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

NOTE: This endpoint should be called only by the users to get values for their runs via API.
Further we need to validate if the user has access to the run_id, otherwise we should not return the output urls.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.dub.get_dubbed_run_info(&Some(1), None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">get_dubbing_runs_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, GetDubbingRunsResultsDubbingResultsPostResponseValue&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .get_dubbing_runs_results(
            &GetDubbingRunsResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">get_dubbed_run_transcript</a>(run_id: Option&lt;i64&gt;, language: Languages, format_type: Option&lt;Option&lt;TranscriptFileFormat&gt;&gt;, data_type: Option&lt;Option&lt;TranscriptDataType&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, String&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .get_dubbed_run_transcript(
            &Some(1),
            &Languages(1),
            &GetDubbedRunTranscriptQueryRequest {
                format_type: None,
                data_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**format_type:** `Option<TranscriptFileFormat>` — Format to use for the transcription. Either `srt`, `vtt` or `txt`. Defaults to `txt`.
    
</dd>
</dl>

<dl>
<dd>

**data_type:** `Option<TranscriptDataType>` — Data type for the transcription being returned. Returns the raw data of the transcription or a presigned url for the file that holds the transcription data.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">get_dubbed_output_in_alt_format</a>(run_id: Option&lt;i64&gt;, language: Languages, request: DubbedOutputInAltFormatRequestPayload) -> Result&lt;GetDubbedOutputInAltFormatDubAltFormatRunIDLanguagePostResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .get_dubbed_output_in_alt_format(
            &Some(1),
            &Languages(1),
            &DubbedOutputInAltFormatRequestPayload {
                output_format: DubbedOutputInAltFormatRequestPayloadOutputFormat::AudioOutputType(
                    AudioOutputType::Flac,
                ),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**output_format:** `DubbedOutputInAltFormatRequestPayloadOutputFormat` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">get_dubbed_output_in_alt_format_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .get_dubbed_output_in_alt_format_status(
            &"task_id".to_string(),
            &GetDubbedOutputInAltFormatStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">poll_discord_dub_task</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .poll_discord_dub_task(
            &"task_id".to_string(),
            &PollDiscordDubTaskQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dub.<a href="/src/api/resources/dub/client.rs">poll_twitter_dub_task</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dub
        .poll_twitter_dub_task(
            &"task_id".to_string(),
            &PollTwitterDubTaskQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Folders
<details><summary><code>client.folders.<a href="/src/api/resources/folders/client.rs">list_folders</a>(limit: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, search_query: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;Folder&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .folders
        .list_folders(
            &ListFoldersQueryRequest {
                limit: None,
                search_query: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**limit:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**search_query:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.folders.<a href="/src/api/resources/folders/client.rs">create_folder</a>(request: CreateFolderPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .folders
        .create_folder(
            &CreateFolderPayload {
                folder_name: "folder_name".to_string(),
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**folder_name:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Languages
<details><summary><code>client.languages.<a href="/src/api/resources/languages/client.rs">get_source_languages</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;LanguagePydanticModel&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .languages
        .get_source_languages(&GetSourceLanguagesQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.languages.<a href="/src/api/resources/languages/client.rs">get_target_languages</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;LanguagePydanticModel&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .languages
        .get_target_languages(&GetTargetLanguagesQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Story
<details><summary><code>client.story.<a href="/src/api/resources/story/client.rs">create_story</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;CreateStoryStoryPostResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .story
        .create_story(
            &CreateStoryRequest {
                file: b"test file content".to_vec(),
                source_language: Languages(1),
                title: None,
                description: None,
                narrator_voice_id: None,
                folder_id: None,
                chosen_dictionaries: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.story.<a href="/src/api/resources/story/client.rs">setup_story</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;SetupStoryStorySetupPostResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .story
        .setup_story(
            &SetupStoryRequest {
                file: b"test file content".to_vec(),
                source_language: Languages(1),
                title: None,
                description: None,
                narrator_voice_id: None,
                folder_id: None,
                chosen_dictionaries: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.story.<a href="/src/api/resources/story/client.rs">get_story_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .story
        .get_story_status(
            &"task_id".to_string(),
            &GetStoryStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.story.<a href="/src/api/resources/story/client.rs">get_story_run_info</a>(run_id: Option&lt;i64&gt;, include_transcript: Option&lt;Option&lt;Option&lt;bool&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .story
        .get_story_run_info(
            &Some(1),
            &GetStoryRunInfoQueryRequest {
                include_transcript: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**include_transcript:** `Option<Option<bool>>` — Whether to include the transcription in the response for fetching the result of a Stories run.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.story.<a href="/src/api/resources/story/client.rs">get_stories_runs_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, std::collections::HashMap&lt;String, serde_json::Value&gt;&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .story
        .get_stories_runs_results(
            &GetStoriesRunsResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TranslatedStory
<details><summary><code>client.translated_story.<a href="/src/api/resources/translated_story/client.rs">create_translation_for_existing_story</a>(run_id: Option&lt;i64&gt;, request: CreateTranslationForExistingStoryRequestPayload) -> Result&lt;AddTargetLanguageOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translated_story
        .create_translation_for_existing_story(
            &Some(1),
            &CreateTranslationForExistingStoryRequestPayload {
                target_language: Languages(1),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**target_language:** `Languages` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translated_story.<a href="/src/api/resources/translated_story/client.rs">get_translated_story_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translated_story
        .get_translated_story_status(
            &"task_id".to_string(),
            &GetTranslatedStoryStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translated_story.<a href="/src/api/resources/translated_story/client.rs">get_translated_story_run_info</a>(run_id: Option&lt;i64&gt;, target_language: Languages, include_transcript: Option&lt;Option&lt;Option&lt;bool&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translated_story
        .get_translated_story_run_info(
            &Some(1),
            &Languages(1),
            &GetTranslatedStoryRunInfoQueryRequest {
                include_transcript: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**target_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**include_transcript:** `Option<Option<bool>>` — Whether to include the transcription in the response for fetching the result of a Stories Translation run.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TextToAudio
<details><summary><code>client.text_to_audio.<a href="/src/api/resources/text_to_audio/client.rs">create_text_to_audio</a>(request: CreateTextToAudioRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineCallResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_audio
        .create_text_to_audio(
            &CreateTextToAudioRequestPayload {
                prompt: "prompt".to_string(),
                run_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                duration: None,
                audio_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**prompt:** `String` — The text to be converted to audio.
    
</dd>
</dl>

<dl>
<dd>

**duration:** `Option<f64>` — The desired duration of the audio.
    
</dd>
</dl>

<dl>
<dd>

**audio_type:** `Option<TextToAudioType>` — The audio type preference.
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_audio.<a href="/src/api/resources/text_to_audio/client.rs">get_text_to_audio_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_audio
        .get_text_to_audio_status(
            &"task_id".to_string(),
            &GetTextToAudioStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_audio.<a href="/src/api/resources/text_to_audio/client.rs">get_text_to_sound_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, TextToAudioResult&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_audio
        .get_text_to_sound_results(
            &GetTextToSoundResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TextToVoice
<details><summary><code>client.text_to_voice.<a href="/src/api/resources/text_to_voice/client.rs">create_text_to_voice</a>(request: CreateTextToVoiceRequestPayload) -> Result&lt;OrchestratorPipelineCallResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_voice
        .create_text_to_voice(
            &CreateTextToVoiceRequestPayload {
                text: "text".to_string(),
                voice_description: "voice_description".to_string(),
                project_name: None,
                project_description: None,
                folder_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` — The text to be converted to voice
    
</dd>
</dl>

<dl>
<dd>

**voice_description:** `String` — Description to use for the created voice
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_voice.<a href="/src/api/resources/text_to_voice/client.rs">get_text_to_voice_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_voice
        .get_text_to_voice_status(
            &"task_id".to_string(),
            &GetTextToVoiceStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_voice.<a href="/src/api/resources/text_to_voice/client.rs">get_text_to_voice_result</a>(run_id: Option&lt;i64&gt;) -> Result&lt;GetTextToVoiceResultOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_voice
        .get_text_to_voice_result(&Some(1), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TextToSpeech
<details><summary><code>client.text_to_speech.<a href="/src/api/resources/text_to_speech/client.rs">tts</a>(request: CreateStreamTtsRequestPayload) -> Result&lt;Vec&lt;u8&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_speech
        .tts(
            &CreateStreamTtsRequestPayload {
                text: "foo".to_string(),
                language: CreateStreamTtsRequestPayloadLanguage::ArKw,
                voice_id: 1,
                speech_model: None,
                user_instructions: None,
                enhance_named_entities_pronunciation: None,
                output_configuration: None,
                voice_settings: None,
                inference_options: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**text:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**language:** `CreateStreamTtsRequestPayloadLanguage` 
    
</dd>
</dl>

<dl>
<dd>

**voice_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**speech_model:** `Option<CreateStreamTtsRequestPayloadSpeechModel>` 
    
</dd>
</dl>

<dl>
<dd>

**user_instructions:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**enhance_named_entities_pronunciation:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**output_configuration:** `Option<StreamTtsOutputConfiguration>` 
    
</dd>
</dl>

<dl>
<dd>

**voice_settings:** `Option<StreamTtsVoiceSettings>` 
    
</dd>
</dl>

<dl>
<dd>

**inference_options:** `Option<StreamTtsInferenceOptions>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_speech.<a href="/src/api/resources/text_to_speech/client.rs">create_tts</a>(request: CreateTtsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;CreateTtsOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_speech
        .create_tts(
            &CreateTtsRequestPayload {
                text: "text".to_string(),
                voice_id: 1,
                language: Languages(1),
                run_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                gender: None,
                age: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**voice_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Gender>` 
    
</dd>
</dl>

<dl>
<dd>

**age:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_speech.<a href="/src/api/resources/text_to_speech/client.rs">get_tts_result</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_speech
        .get_tts_result(
            &"task_id".to_string(),
            &GetTtsResultQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_speech.<a href="/src/api/resources/text_to_speech/client.rs">get_tts_run_info</a>(run_id: Option&lt;i64&gt;, output_type: Option&lt;Option&lt;String&gt;&gt;) -> Result&lt;GetTtsRunInfoTtsResultRunIDGetResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the result of a Text To Speech (TTS) run.

This endpoint validates the provided `run_id` and fetches the corresponding TTS-generated audio.
The user must have valid access to the run. The function supports two output formats:
- `RAW_BYTES`: Streams the audio file directly.
- `FILE_URL`: Returns a pre-signed URL to download the audio file.

Args:
    run_id (int): Unique identifier for the TTS run.
    api_key_obj (dict): API key object used for authentication and storage preferences.
    traceparent (Optional[str]): Traceparent header for distributed tracing.
    output_type (OutputType, optional): Determines the output format. Defaults to `RAW_BYTES`.

Returns:
    StreamingResponse | GetTTSOut:
        - If `output_type = RAW_BYTES`: A streaming response containing the TTS-generated audio in FLAC format.
        - If `output_type = FILE_URL`: A URL pointing to the stored TTS-generated audio file.

Raises:
    HTTPException:
        - 400 BAD REQUEST if the run ID is invalid or does not belong to a TTS process.
        - 500 INTERNAL SERVER ERROR if fetching or streaming the audio fails.

Assumptions:
    - The user has valid access to the `run_id`.
    - The `run_id` corresponds to a valid TTS run.
    - There is only **one** dialogue associated with the given `run_id`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_speech
        .get_tts_run_info(
            &Some(1),
            &GetTtsRunInfoQueryRequest { output_type: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**output_type:** `Option<String>` — Output format for the Text To Speech result
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_speech.<a href="/src/api/resources/text_to_speech/client.rs">get_tts_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, GetTtsResultsTtsResultsPostResponseValue&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_speech
        .get_tts_results(
            &GetTtsResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.text_to_speech.<a href="/src/api/resources/text_to_speech/client.rs">get_tts_result_discord</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .text_to_speech
        .get_tts_result_discord(
            &"task_id".to_string(),
            &GetTtsResultDiscordQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Translation
<details><summary><code>client.translation.<a href="/src/api/resources/translation/client.rs">translation_stream</a>(request: CreateTranslationStreamRequestPayload) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translation
        .translation_stream(
            &CreateTranslationStreamRequestPayload {
                source_language: Languages(1),
                target_language: Languages(1),
                text: "text".to_string(),
                formality: None,
                gender: None,
                team_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**source_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**target_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**formality:** `Option<Option<Formalities>>` 
    
</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Option<Gender>>` 
    
</dd>
</dl>

<dl>
<dd>

**team_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translation.<a href="/src/api/resources/translation/client.rs">create_translation</a>(request: CreateTranslationRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translation
        .create_translation(
            &CreateTranslationRequestPayload {
                texts: vec!["texts".to_string()],
                source_language: Languages(1),
                target_language: Languages(1),
                run_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                age: None,
                formality: None,
                gender: None,
                chosen_dictionaries: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**texts:** `Vec<String>` 
    
</dd>
</dl>

<dl>
<dd>

**age:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**formality:** `Option<Formalities>` 
    
</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Gender>` 
    
</dd>
</dl>

<dl>
<dd>

**source_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**target_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**chosen_dictionaries:** `Option<Option<Vec<i64>>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translation.<a href="/src/api/resources/translation/client.rs">get_translation_task_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translation
        .get_translation_task_status(
            &"task_id".to_string(),
            &GetTranslationTaskStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translation.<a href="/src/api/resources/translation/client.rs">get_translation_result</a>(run_id: Option&lt;i64&gt;) -> Result&lt;TranslationResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

NOTE: This endpoint should be called only by the users to get values for their runs via API.
Further we need to validate if the user has access to the run_id, otherwise we should not return the output urls.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translation
        .get_translation_result(&Some(1), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translation.<a href="/src/api/resources/translation/client.rs">get_translation_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, TranslationResult&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translation
        .get_translation_results(
            &GetTranslationResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Transcription
<details><summary><code>client.transcription.<a href="/src/api/resources/transcription/client.rs">create_transcription</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineCallResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transcription
        .create_transcription(
            &CreateTranscriptionRequest {
                media_file: b"test file content".to_vec(),
                file: b"test file content".to_vec(),
                language: Languages(1),
                media_url: None,
                audio_url: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transcription.<a href="/src/api/resources/transcription/client.rs">get_transcription_task_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transcription
        .get_transcription_task_status(
            &"task_id".to_string(),
            &GetTranscriptionTaskStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transcription.<a href="/src/api/resources/transcription/client.rs">get_transcription_result</a>(run_id: Option&lt;i64&gt;, word_level_timestamps: Option&lt;Option&lt;Option&lt;bool&gt;&gt;&gt;) -> Result&lt;TranscriptionResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

NOTE: This endpoint should be called only by the users to get values for their runs via API.
Further we need to validate if the user has access to the run_id, otherwise we should not return the output urls.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transcription
        .get_transcription_result(
            &Some(1),
            &GetTranscriptionResultQueryRequest {
                word_level_timestamps: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>

<dl>
<dd>

**word_level_timestamps:** `Option<Option<bool>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.transcription.<a href="/src/api/resources/transcription/client.rs">get_transcription_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, TranscriptionResult&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .transcription
        .get_transcription_results(
            &GetTranscriptionResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TranslatedTts
<details><summary><code>client.translated_tts.<a href="/src/api/resources/translated_tts/client.rs">create_translated_tts</a>(request: CreateTranslatedTtsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;CreateTranslatedTtsOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translated_tts
        .create_translated_tts(
            &CreateTranslatedTtsRequestPayload {
                text: "text".to_string(),
                voice_id: 1,
                source_language: Languages(1),
                target_language: Languages(1),
                run_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                age: None,
                formality: None,
                gender: None,
                chosen_dictionaries: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**text:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**voice_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**age:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**formality:** `Option<Option<Formalities>>` 
    
</dd>
</dl>

<dl>
<dd>

**gender:** `Option<Option<Gender>>` 
    
</dd>
</dl>

<dl>
<dd>

**source_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**target_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**chosen_dictionaries:** `Option<Option<Vec<i64>>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.translated_tts.<a href="/src/api/resources/translated_tts/client.rs">get_translated_tts_task_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;OrchestratorPipelineResult, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .translated_tts
        .get_translated_tts_task_status(
            &"task_id".to_string(),
            &GetTranslatedTtsTaskStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Streaming
<details><summary><code>client.streaming.<a href="/src/api/resources/streaming/client.rs">create_stream</a>(request: CreateStreamRequestPayload) -> Result&lt;CreateStreamOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .streaming
        .create_stream(
            &CreateStreamRequestPayload {
                name: None,
                description: None,
                initial_delay: None,
                timeout_in_mins: None,
                voices: vec![1],
                dictionaries: vec![1],
                config: ConfigStream {
                    pipeline: None,
                    mixing: None,
                },
                source_stream: SourceStream {
                    language: Languages(1),
                    url: "url".to_string(),
                    category: None,
                    passphrase: None,
                    streamid: None,
                    number_of_streams: None,
                    audio_stream: None,
                    background_audio_stream: None,
                    latency: None,
                    relay_input: None,
                },
                target_streams: vec![TargetStream {
                    languages: vec![Languages(1)],
                    url: "url".to_string(),
                    r#type: StreamType(1),
                    passphrase: None,
                    streamid: None,
                    pids: None,
                    transcode_video: None,
                    embed_subtitles: None,
                    audio_codec: None,
                    audio_bitrate: None,
                    audio_channel_layout: None,
                    latency: None,
                    constant_bitrate: None,
                    relay_output: None,
                }],
                start_time: None,
                end_time: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.streaming.<a href="/src/api/resources/streaming/client.rs">get_stream_result</a>(stream_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .streaming
        .get_stream_result(1, &GetStreamResultQueryRequest2 { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**stream_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.streaming.<a href="/src/api/resources/streaming/client.rs">destroy_stream</a>(stream_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .streaming
        .destroy_stream(1, &DestroyStreamQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**stream_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.streaming.<a href="/src/api/resources/streaming/client.rs">patch_stream_data</a>(stream_id: i64, request: UpdateStreamDataRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .streaming
        .patch_stream_data(
            1,
            &UpdateStreamDataRequestPayload {
                run_id: None,
                start_time: None,
                end_time: None,
                payload: None,
                target_languages: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**stream_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**start_time:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**end_time:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**payload:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` 
    
</dd>
</dl>

<dl>
<dd>

**target_languages:** `Option<Option<Vec<Languages>>>` 
    
</dd>
</dl>

<dl>
<dd>

**timezone:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.streaming.<a href="/src/api/resources/streaming/client.rs">get_probe_stream</a>(request: GetProbeStreamIn, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;GetProbeStreamOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .streaming
        .get_probe_stream(
            &GetProbeStreamRequest {
                body: GetProbeStreamIn {
                    url: "url".to_string(),
                    passphrase: None,
                    stream_id: None,
                },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## VoiceCloning
<details><summary><code>client.voice_cloning.<a href="/src/api/resources/voice_cloning/client.rs">list_voices</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;ListVoicesListVoicesGetResponseItem&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .voice_cloning
        .list_voices(&ListVoicesQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.voice_cloning.<a href="/src/api/resources/voice_cloning/client.rs">create_custom_voice</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;CreateCustomVoiceOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .voice_cloning
        .create_custom_voice(
            &CreateCustomVoiceRequest {
                file: b"test file content".to_vec(),
                voice_name: "voice_name".to_string(),
                gender: Gender(1),
                description: None,
                publish_voice_to_market_place: None,
                age: None,
                language: None,
                enhance_audio: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Dictionaries
<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">get_dictionaries</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;DictionaryWithTerms&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .get_dictionaries(&GetDictionariesQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">create_dictionary_from_file</a>(run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .create_dictionary_from_file(
            &CreateDictionaryFromFileRequest {
                dictionary_file: b"test file content".to_vec(),
                dictionary_name: "dictionary_name".to_string(),
                dictionary_description: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">get_dictionary_info</a>(dictionary_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;DictionaryWithTerms, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .get_dictionary_info(1, &GetDictionaryInfoQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dictionary_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">delete_dictionary</a>(dictionary_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .delete_dictionary(1, &DeleteDictionaryQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dictionary_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">get_dictionary_details</a>(dictionary_id: i64, limit: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;, search_term: Option&lt;Option&lt;Option&lt;String&gt;&gt;&gt;, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;DictionaryWithTerms, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .get_dictionary_details(
            1,
            &GetDictionaryDetailsQueryRequest {
                limit: None,
                search_term: None,
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dictionary_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**limit:** `Option<Option<i64>>` — Limit how many terms are returned when fetching the dictionary details.
    
</dd>
</dl>

<dl>
<dd>

**search_term:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">add_term_to_dictionary</a>(dictionary_id: i64, request: AddDictionaryTermPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .add_term_to_dictionary(
            1,
            &AddDictionaryTermPayload {
                translations: vec![TermTranslationInput {
                    translation: "translation".to_string(),
                    language: Languages(1),
                }],
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dictionary_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Vec<TermTranslationInput>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">update_term_translation_in_dictionary_using_term_id</a>(dictionary_id: i64, term_id: i64, request: UpdateTermTranslationsPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .update_term_translation_in_dictionary_using_term_id(
            1,
            1,
            &UpdateTermTranslationsPayload {
                translations: vec![TermTranslationInput {
                    translation: "translation".to_string(),
                    language: Languages(1),
                }],
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dictionary_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**term_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**translations:** `Vec<TermTranslationInput>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.dictionaries.<a href="/src/api/resources/dictionaries/client.rs">delete_dictionary_term</a>(dictionary_id: i64, term_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;serde_json::Value, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .dictionaries
        .delete_dictionary_term(
            1,
            1,
            &DeleteDictionaryTermQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**dictionary_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**term_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ProjectSetup
<details><summary><code>client.project_setup.<a href="/src/api/resources/project_setup/client.rs">create_project</a>(request: CreateProjectSetupRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;CreateProjectSetupOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a new project setup with transcription capabilities.

This endpoint allows users to create a new project by providing media content
(either as a file upload or URL), specifying source and target languages, and
other project metadata. The function validates inputs, checks file size limitations,
and initiates the project setup process.

Args:
    request_payload (CreateProjectSetupRequestPayload): Complete project configuration
            including media URL, source/target languages, project metadata, and
            processing preferences such as audio track selection and dictionary choices.
        api_key_obj_and_subscription: Dependency injection providing validated API key
            object and associated subscription details for authorization and usage
            limit enforcement.
        traceparent (str | None, optional): OpenTelemetry trace parent header for
            distributed tracing across microservices. Enables request correlation
            and performance monitoring throughout the processing pipeline.

Returns:
    Project setup response with project details and processing status.

Raises:
    HTTPException:
        - 400: If neither media_file nor media_url is provided
        - 400: If uploaded file has no filename
        - 413: If uploaded file exceeds size limit
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .project_setup
        .create_project(
            &CreateProjectSetupRequestPayload {
                media_url: "media_url".to_string(),
                source_language: Languages(1),
                target_languages: vec![Languages(1)],
                run_id: None,
                project_name: None,
                project_description: None,
                folder_id: None,
                selected_audio_tracks: None,
                add_output_as_an_audio_track: None,
                chosen_dictionaries: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**project_name:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**project_description:** `Option<Option<String>>` 
    
</dd>
</dl>

<dl>
<dd>

**folder_id:** `Option<Option<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**media_url:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**source_language:** `Languages` 
    
</dd>
</dl>

<dl>
<dd>

**target_languages:** `Vec<Languages>` 
    
</dd>
</dl>

<dl>
<dd>

**selected_audio_tracks:** `Option<Option<Vec<i64>>>` 
    
</dd>
</dl>

<dl>
<dd>

**add_output_as_an_audio_track:** `Option<Option<bool>>` 
    
</dd>
</dl>

<dl>
<dd>

**chosen_dictionaries:** `Option<Vec<i64>>` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.project_setup.<a href="/src/api/resources/project_setup/client.rs">create_project_setup_task_status</a>(task_id: String, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;GetCreateProjectSetupResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .project_setup
        .create_project_setup_task_status(
            &"task_id".to_string(),
            &CreateProjectSetupTaskStatusQueryRequest { run_id: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**task_id:** `String` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.project_setup.<a href="/src/api/resources/project_setup/client.rs">get_project_setup_result</a>(run_id: Option&lt;i64&gt;) -> Result&lt;Option&lt;GetCreateProjectSetupResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the final result of a completed project setup.

This endpoint provides access to the final results of a completed project setup.
It verifies that the authenticated user has access to the requested run_id and
validates that the run is of the correct type (`DUB_PROJECT`) before returning results.

Note:
    This endpoint should only be called by users to retrieve their run results via API.
    Access validation is performed to ensure users can only access their own runs.

Args:
    run_id: Positive integer ID of the project setup run.
    api_key_obj: API key authentication data from dependency.
    traceparent: OpenTelemetry trace header for distributed tracing.

Returns:
    GetCreateProjectSetupResponse: Project setup results including run details.

Raises:
    HTTPException:
        - 404: If the run_id is not found
        - 400: If the run type is not valid for this endpoint (must be DUB_PROJECT)
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .project_setup
        .get_project_setup_result(&Some(1), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<i64>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.project_setup.<a href="/src/api/resources/project_setup/client.rs">get_project_setup_runs_results</a>(request: RunIDsRequestPayload, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;Vec&lt;GetCreateProjectSetupResponse&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .project_setup
        .get_project_setup_runs_results(
            &GetProjectSetupRunsResultsRequest {
                body: RunIDsRequestPayload { run_ids: vec![1] },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## DeprecatedStreaming
<details><summary><code>client.deprecated_streaming.<a href="/src/api/resources/deprecated_streaming/client.rs">create_stream</a>(request: CreateStreamRequestPayload) -> Result&lt;CreateStreamOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .deprecated_streaming
        .create_stream(
            &CreateStreamRequestPayload {
                name: None,
                description: None,
                initial_delay: None,
                timeout_in_mins: None,
                voices: vec![1],
                dictionaries: vec![1],
                config: ConfigStream {
                    pipeline: None,
                    mixing: None,
                },
                source_stream: SourceStream {
                    language: Languages(1),
                    url: "url".to_string(),
                    category: None,
                    passphrase: None,
                    streamid: None,
                    number_of_streams: None,
                    audio_stream: None,
                    background_audio_stream: None,
                    latency: None,
                    relay_input: None,
                },
                target_streams: vec![TargetStream {
                    languages: vec![Languages(1)],
                    url: "url".to_string(),
                    r#type: StreamType(1),
                    passphrase: None,
                    streamid: None,
                    pids: None,
                    transcode_video: None,
                    embed_subtitles: None,
                    audio_codec: None,
                    audio_bitrate: None,
                    audio_channel_layout: None,
                    latency: None,
                    constant_bitrate: None,
                    relay_output: None,
                }],
                start_time: None,
                end_time: None,
                timezone: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deprecated_streaming.<a href="/src/api/resources/deprecated_streaming/client.rs">get_stream_result</a>(stream_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .deprecated_streaming
        .get_stream_result(1, &GetStreamResultQueryRequest2 { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**stream_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deprecated_streaming.<a href="/src/api/resources/deprecated_streaming/client.rs">stop_stream</a>(stream_id: i64, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;std::collections::HashMap&lt;String, serde_json::Value&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .deprecated_streaming
        .stop_stream(1, &StopStreamQueryRequest { run_id: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**stream_id:** `i64` 
    
</dd>
</dl>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.deprecated_streaming.<a href="/src/api/resources/deprecated_streaming/client.rs">get_probe_stream</a>(request: GetProbeStreamIn, run_id: Option&lt;Option&lt;Option&lt;i64&gt;&gt;&gt;) -> Result&lt;GetProbeStreamOut, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use camb_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .deprecated_streaming
        .get_probe_stream(
            &GetProbeStreamRequest {
                body: GetProbeStreamIn {
                    url: "url".to_string(),
                    passphrase: None,
                    stream_id: None,
                },
                run_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**run_id:** `Option<Option<i64>>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>
