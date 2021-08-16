use askama::Template;
use serde::Serialize;
use sqlx::Error;

#[derive(Serialize, Template)]
#[template(path = "trails/index.html")]
pub struct TrailsOrchestrator;

impl TrailsOrchestrator {
    pub async fn find() -> Result<TrailsOrchestrator, Error> {
        Ok(TrailsOrchestrator)
    }
}
