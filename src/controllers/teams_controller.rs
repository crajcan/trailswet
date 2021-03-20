use crate::teams_orchestrator::TeamsOrchestrator;
use crate::{NetflixError, Presenter};
use actix_web::web::Data;
use actix_web::*;
use sqlx::PgPool;

#[get(r#"/teams{tail:.*}"#)]
async fn index(db_pool: Data<PgPool>) -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: TeamsOrchestrator::find(db_pool.get_ref()).await?,
    })
}
