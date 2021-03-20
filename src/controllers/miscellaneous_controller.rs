use crate::home_page_orchestrator::HomePageOrchestrator;
use crate::{NetflixError, Presenter};
use actix_web::web::Data;
use actix_web::*;
use sqlx::PgPool;

#[get(r#"/{tail:.*}"#)]
async fn home(db_pool: Data<PgPool>) -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: HomePageOrchestrator::load(db_pool.get_ref()).await?,
    })
}
