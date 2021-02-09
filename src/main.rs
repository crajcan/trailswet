use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use tera::{Context, Tera};

#[get("/")]
async fn home(view: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let game = GameOrchestrator {
        home_team_name: "Miami Hurricanes".into(),
        away_team_name: "Nebraska Cornhuskers".into(),
    };

    let body = view
        .render("home.html", &Context::from_serialize(&game).unwrap())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

#[get("/home_json")]
async fn home_json() -> Result<HttpResponse, Error> {
    let game = GameOrchestrator {
        home_team_name: "Miami Hurricanes".into(),
        away_team_name: "Nebraska Cornhuskers".into(),
    };

    Ok(HttpResponse::Ok().json(game))
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

        App::new().data(tera).service(home).service(home_json)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
