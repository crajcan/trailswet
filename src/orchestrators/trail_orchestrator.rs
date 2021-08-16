use crate::trail::Trail;
use askama::Template;
use serde::Serialize;
use sqlx::Error;

#[derive(Serialize, Template)]
#[template(path = "trails/show.html")]
pub struct TrailOrchestrator {
    trail: Trail,
}

impl TrailOrchestrator {
    pub async fn find(trail_id: i32) -> Result<TrailOrchestrator, Error> {
        let trail = Trail::find(trail_id).await?;

        Ok(TrailOrchestrator {
            trail,
        })
    }
}
