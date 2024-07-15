use std::ops::Deref;

use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use crate::repository::user_views::UserViewsRepositoryOption;

pub struct UserViewsRepositoryExtractor(UserViewsRepositoryOption);

impl Deref for UserViewsRepositoryExtractor {
    type Target = UserViewsRepositoryOption;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for UserViewsRepositoryExtractor {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for UserViewsRepositoryExtractor {
    type Rejection = ();

    async fn from_request_parts<'a, 'b>(_: &'a mut Parts, _: &'b T) -> Result<Self, Self::Rejection> {
        Ok(Default::default())
    }
}