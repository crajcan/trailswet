use actix_web::{error, get, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
struct GameOrchestrator {
    home_team_name: String,
    away_team_name: String,
}

impl Responder for GameOrchestrator {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let path: PathBuf = req.match_info().query("tail").parse().unwrap();

        ready(match path.to_str().unwrap() {
            "" | "/" => {
                /*
                let body = view
                    .render("home.html", &Context::from_serialize(&self).unwrap())
                    .map_err(|_| error::ErrorInternalServerError("Template error"))?;

                Ok(HttpResponse::Ok().content_type("text/html").body(body))
                */
                Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .json(self))
            }
            ".json" => Ok(HttpResponse::Ok()
                .content_type("application/json")
                .json(self)),
            _ => Err(error::ErrorNotFound("Resource Not Found")),
        })
    }
}

#[get("home{tail:.*}")]
async fn home() -> impl Responder {
    GameOrchestrator {
        home_team_name: "Miami Hurricanes".into(),
        away_team_name: "Nebraska Cornhuskers".into(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(home))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
