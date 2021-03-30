use crate::game_orchestrator::GameOrchestrator;
use crate::games_orchestrator::GamesOrchestrator;
use crate::{NetflixError, Presenter};
use actix_web::web::{Path};
use actix_web::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

#[get(r#"/game/{id:\d+}{tail:.*}"#)]
async fn show(
   path_params: Path<PathParams>
) -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: GameOrchestrator::find(path_params.id).await?,
    })
}

#[get(r#"/games{tail:.*}"#)]
async fn index() -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: GamesOrchestrator::find().await?,
    })
}
