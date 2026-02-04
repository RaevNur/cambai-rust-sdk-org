use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct FoldersClient {
    pub http_client: HttpClient,
}

impl FoldersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    pub async fn list_folders(
        &self,
        request: &ListFoldersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<Folder>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "folders",
                None,
                QueryBuilder::new()
                    .serialize("limit", request.limit.clone())
                    .serialize("search_query", request.search_query.clone())
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }

    pub async fn create_folder(
        &self,
        request: &CreateFolderPayload,
        options: Option<RequestOptions>,
    ) -> Result<serde_json::Value, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "folders/create",
                Some(serde_json::to_value(request).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
