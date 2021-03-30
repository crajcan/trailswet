use askama::Template;
use serde::Serialize;
use sqlx::{Error};

#[derive(Serialize, Template)]
#[template(path = "miscellaneous/home.html")]
pub struct HomePageOrchestrator;

impl HomePageOrchestrator {
    pub async fn load() -> Result<HomePageOrchestrator, Error> {
        Ok(HomePageOrchestrator)
    }
}
