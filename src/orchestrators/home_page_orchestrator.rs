use askama::Template;
use serde::Serialize;
use sqlx::{Error, PgPool};

#[derive(Serialize, Template)]
#[template(path = "miscellaneous/home.html")]
pub struct HomePageOrchestrator;

impl HomePageOrchestrator {
    pub async fn load(_pool: &PgPool) -> Result<HomePageOrchestrator, Error> {
        Ok(HomePageOrchestrator)
    }
}
