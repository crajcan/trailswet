use crate::Presenter;
use actix_web::*;
use crate::game_orchestrator::GameOrchestrator;

#[get("/show{tail:.*}")]
async fn show() -> impl Responder {
    let orchestrator = GameOrchestrator::find(1);

    Presenter {
        resource: orchestrator,
        template: "games/show.html",
    }
}
