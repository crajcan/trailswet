use crate::game_orchestrator::GameOrchestrator;
use crate::Presenter;
use actix_web::*;

#[get("/show{tail:.*}")]
async fn show() -> impl Responder {
    let orchestrator = GameOrchestrator::find(1);

    Presenter {
        resource: orchestrator,
        template: "games/show.html",
    }
}
