use askama::Template;
use serde::Serialize;
use sqlx::{Error};

#[derive(Serialize, Template)]
#[template(path = "games/index.html")]
pub struct GamesOrchestrator;

impl GamesOrchestrator {
    pub async fn find() -> Result<GamesOrchestrator, Error> {
        Ok(GamesOrchestrator)
    }
}
