use ::axum::{routing::get, Router};
use log_layer::LogLayer;
use routes::MetricsRouter;

mod health;
mod configuration;
mod routes;
mod web;
mod repository;
mod axum;

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health::health))
        .register_metrics_routes()
        .layer(LogLayer::new())
}