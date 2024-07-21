use axum::{http::StatusCode, Json};
use or_status_code::OrInternalServerError;
use serde::Serialize;

use crate::axum::extractors::user_views_repository::UserViewsRepositoryExtractor;
use crate::repository::user_views::UserViewsRepository;

#[derive(Serialize)]
pub struct PopularUsersResponse {
    #[serde(rename = "u")]
    pub users: Vec<PopularUser>,
}

#[derive(Serialize)]
pub struct PopularUser {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "c")]
    pub score: u32,
}

pub async fn popular_users(
    user_views_repository: UserViewsRepositoryExtractor
) -> Result<Json<PopularUsersResponse>, StatusCode> {
    let users = user_views_repository
        .popular()
        .await
        .or_internal_server_error()?;

    let response = PopularUsersResponse {
        users: users.into_iter().map(|user| PopularUser {
            id: user.id,
            score: user.score,
        }).collect(),
    };

    Ok(Json(response))
}