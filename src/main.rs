use actix_service::Service;
use actix_web::{App, Error, get, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::future::{FutureExt, ready, Ready};
use serde::Serialize;
use std::path::PathBuf;

#[get("index{tail:.*}")]
async fn index() -> impl Responder {
    Todo {
      id: 42,
      description: "Walk the doggo".into(),
      done: true
    }
}

#[derive(Serialize)]
struct GameOrchestrator {
    home_team_name: String,
    away_team_name: String,
}


// this struct will be used to represent database record
#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

// implementation of Actix Responder for Todo struct so we can return Todo from action handler
impl Responder for Todo {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let path: PathBuf = req.match_info().query("tail").parse().unwrap();
        println!("path: {:?}", path);

        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body)
        ))
    }
}


// `std::result::Result<ServiceResponse, actix_web::Error>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("before handler, path: {}", req.path());
                srv.call(req).map(|res| {
                    println!("processing response");
                    res
                })
            })
            .service(index)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
