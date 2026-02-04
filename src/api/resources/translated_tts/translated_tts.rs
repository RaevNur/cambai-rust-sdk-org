use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TranslatedTtsClient {
    pub http_client: HttpClient,
}

impl TranslatedTtsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_translated_tts(
        &self,
        request: &CreateTranslatedTtsRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<CreateTranslatedTtsOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "translated-tts",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_translated_tts_task_status(
        &self,
        task_id: &String,
        request: &GetTranslatedTtsTaskStatusQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<OrchestratorPipelineResult, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("translated-tts/{}", task_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
