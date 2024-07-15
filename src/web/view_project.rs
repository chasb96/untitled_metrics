use axum::http::StatusCode;
use json_or_protobuf::JsonOrProtobuf;
use prost::Message;
use serde::Deserialize;

use crate::axum::extractors::project_views_repository::ProjectViewsRepositoryExtractor;
use crate::repository::project_views::ProjectViewsRepository;

#[derive(Deserialize, Message)]
pub struct ViewProjectRequest {
    #[prost(string, tag = "1")]
    pub project_id: String,
}

pub async fn view_project(
    project_views_repository: ProjectViewsRepositoryExtractor,
    request: JsonOrProtobuf<ViewProjectRequest>
) -> Result<StatusCode, StatusCode> {
    let (request, _) = request.decompose();

    project_views_repository
        .increment_view_count(&request.project_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}