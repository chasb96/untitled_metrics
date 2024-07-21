use serde::Deserialize;

mod error;
mod deadpool;
mod mongo;

pub mod project_views;
pub mod user_views;

#[derive(Deserialize)]
pub struct MetricUser {
    pub id: String,
    pub score: u32,
}

#[derive(Deserialize)]
pub struct MetricProject {
    pub id: String,
    pub score: u32,
}