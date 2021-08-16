use crate::teams_orchestrator::TeamsOrchestrator;
use crate::{NetflixError, Presenter};
use actix_web::*;

#[get(r#"/teams{tail:.*}"#)]
async fn index() -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: TeamsOrchestrator::find().await?,
    })
}
