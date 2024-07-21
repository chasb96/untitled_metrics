use axum::{http::StatusCode, Json};
use or_status_code::OrInternalServerError;
use serde::Serialize;

use crate::axum::extractors::project_views_repository::ProjectViewsRepositoryExtractor;
use crate::repository::project_views::ProjectViewsRepository;

#[derive(Serialize)]
pub struct PopularProjectsResponse {
    #[serde(rename = "u")]
    pub projects: Vec<PopularProject>,
}

#[derive(Serialize)]
pub struct PopularProject {
    #[serde(rename = "i")]
    pub id: String,
    #[serde(rename = "c")]
    pub score: u32,
}

pub async fn popular_projects(
    project_views_repository: ProjectViewsRepositoryExtractor
) -> Result<Json<PopularProjectsResponse>, StatusCode> {
    let projects = project_views_repository
        .popular()
        .await
        .or_internal_server_error()?;

    let response = PopularProjectsResponse {
        projects: projects.into_iter().map(|project| PopularProject {
            id: project.id,
            score: project.score,
        }).collect(),
    };

    Ok(Json(response))
}