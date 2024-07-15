use axum::http::StatusCode;
use json_or_protobuf::JsonOrProtobuf;
use prost::Message;
use serde::Deserialize;

use crate::axum::extractors::user_views_repository::UserViewsRepositoryExtractor;
use crate::repository::user_views::UserViewsRepository;

#[derive(Deserialize, Message)]
pub struct ViewUserRequest {
    #[prost(string, tag = "1")]
    pub user_id: String,
}

pub async fn view_user(
    user_views_repository: UserViewsRepositoryExtractor,
    request: JsonOrProtobuf<ViewUserRequest>
) -> Result<StatusCode, StatusCode> {
    let (request, _) = request.decompose();

    user_views_repository
        .increment_view_count(&request.user_id)
        .await
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}