use crate::home_page_orchestrator::HomePageOrchestrator;
use crate::{NetflixError, Presenter};
use actix_web::*;

#[get(r#"/{tail:.*}"#)]
async fn home() -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: HomePageOrchestrator::load().await?,
    })
}
