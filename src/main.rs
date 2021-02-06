use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use tera::{Tera, Context};
use serde::Serialize;

#[get("/")]
async fn home(view: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let game = &Context::from_serialize(&GameOrchestrator {
                home_team_name: "Miami Hurricanes".into(),
                away_team_name: "Nebraska Cornhuskers".into(),
            }).unwrap();

    let body = view
        .render("home.html", game)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[derive(Serialize)]
struct GameOrchestrator {
    home_team_name: String,
    away_team_name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();

        App::new().data(tera).service(home)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
