use crate::game_orchestrator::GameOrchestrator;
use crate::{NetflixError, Presenter};
use actix_web::web::{Data, Path};
use actix_web::*;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

#[get(r#"/show/{id:\d+}{tail:.*}"#)]
async fn show(
    path_params: Path<PathParams>,
    db_pool: Data<PgPool>,
) -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: GameOrchestrator::find(path_params.id, db_pool.get_ref()).await?,
    })
}
