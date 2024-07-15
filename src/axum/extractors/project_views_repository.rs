use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::repository::project_views::ProjectViewsRepositoryOption;

pub struct ProjectViewsRepositoryExtractor(ProjectViewsRepositoryOption);

impl Deref for ProjectViewsRepositoryExtractor {
    type Target = ProjectViewsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ProjectViewsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for ProjectViewsRepositoryExtractor {
    type Rejection = ();

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}