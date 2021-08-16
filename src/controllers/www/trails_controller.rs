use crate::trail_orchestrator::TrailOrchestrator;
use crate::trails_orchestrator::TrailsOrchestrator;
use crate::{TrailsWetError, Presenter};
use actix_web::web::Path;
use actix_web::*;
use serde::Deserialize;

#[derive(Deserialize)]
struct PathParams {
    id: i32,
}

#[get(r#"/trail/{id:\d+}{tail:.*}"#)]
async fn show(path_params: Path<PathParams>) -> Result<impl Responder, TrailsWetError> {
    Ok(Presenter {
        resource: TrailOrchestrator::find(path_params.id).await?,
    })
}

#[get(r#"/trails{tail:.*}"#)]
async fn index() -> Result<impl Responder, TrailsWetError> {
    Ok(Presenter {
        resource: TrailsOrchestrator::find().await?,
    })
}
