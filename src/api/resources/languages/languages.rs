use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LanguagesClient {
    pub http_client: HttpClient,
}

impl LanguagesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn get_source_languages(
        &self,
        request: &GetSourceLanguagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<LanguagePydanticModel>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "source-languages",
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_target_languages(
        &self,
        request: &GetTargetLanguagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<LanguagePydanticModel>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "target-languages",
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
