use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct DictionariesClient {
    pub http_client: HttpClient,
}

impl DictionariesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn get_dictionaries(
        &self,
        request: &GetDictionariesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<DictionaryWithTerms>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "dictionaries",
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_dictionary_from_file(
        &self,
        request: &CreateDictionaryFromFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_multipart_request(
                Method::POST,
                "dictionaries/create-from-file",
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_dictionary_info(
        &self,
        dictionary_id: i64,
        request: &GetDictionaryInfoQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DictionaryWithTerms, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("dictionaries/{}", dictionary_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn delete_dictionary(
        &self,
        dictionary_id: i64,
        request: &DeleteDictionaryQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("dictionaries/{}", dictionary_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn get_dictionary_details(
        &self,
        dictionary_id: i64,
        request: &GetDictionaryDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<DictionaryWithTerms, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("dictionaries/{}/full-details", dictionary_id),
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("search_term", request.search_term.clone())
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn add_term_to_dictionary(
        &self,
        dictionary_id: i64,
        request: &AddDictionaryTermPayload,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("dictionaries/{}/add-term", dictionary_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn update_term_translation_in_dictionary_using_term_id(
        &self,
        dictionary_id: i64,
        term_id: i64,
        request: &UpdateTermTranslationsPayload,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("dictionaries/term/{}/{}", dictionary_id, term_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn delete_dictionary_term(
        &self,
        dictionary_id: i64,
        term_id: i64,
        request: &DeleteDictionaryTermQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("dictionaries/term/{}/{}", dictionary_id, term_id),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
