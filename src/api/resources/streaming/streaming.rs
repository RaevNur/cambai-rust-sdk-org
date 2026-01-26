use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct StreamingClient {
    pub http_client: HttpClient,
}

impl StreamingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn create_stream(
        &self,
        request: &CreateStreamRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<CreateStreamOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "stream",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    pub async fn get_stream_result(
        &self,
        stream_id: i64,
        request: &StreamingGetStreamResultQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("stream/{}", stream_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn destroy_stream(
        &self,
        stream_id: i64,
        request: &DestroyStreamQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("stream/{}", stream_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn patch_stream_data(
        &self,
        stream_id: i64,
        request: &UpdateStreamDataRequestPayload,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("stream/{}", stream_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_probe_stream(
        &self,
        request: &GetProbeStreamRequest,
        options: Option<RequestOptions>,
    ) -> Result<GetProbeStreamOut, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "stream/probe",
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
