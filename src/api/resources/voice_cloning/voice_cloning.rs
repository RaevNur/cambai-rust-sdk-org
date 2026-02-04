use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VoiceCloningClient {
    pub http_client: HttpClient,
}

impl VoiceCloningClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_voices(
        &self,
        request: &ListVoicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<ListVoicesListVoicesGetResponseItem>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "list-voices",
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_custom_voice(
        &self,
        request: &CreateCustomVoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateCustomVoiceOut, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "create-custom-voice",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
