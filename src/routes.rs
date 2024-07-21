use axum::{routing::post, Router};

use super::web::*;

pub trait MetricsRouter {
    fn register_metrics_routes(self) -> Self;
}

impl MetricsRouter for Router {
    fn register_metrics_routes(self) -> Self {
        self.route("/metrics/users/view", post(view_user))
            .route("/metrics/projects/view", post(view_project))
            .route("/metrics/users/popular", post(popular_users))
            .route("/metrics/projects/popular", post(popular_projects))
    }
}