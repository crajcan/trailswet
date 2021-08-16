#![allow(dead_code)]

use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::*;
use dotenv::dotenv;
use env_logger;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod utils;
use utils::trailswet_error::TrailsWetError;
use utils::presenter::Presenter;

mod controllers;
use controllers::*;

mod orchestrators;
use orchestrators::*;

mod services;

mod models;
use models::*;

#[macro_use]
extern crate lazy_static;
lazy_static! {
    pub static ref DB_POOL: sqlx::PgPool = {
        let database_url =
            env::var("DATABASE_URL").expect("Database URL cannot be resolved from .env file");

        PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(&database_url)
            .unwrap()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let socket_address =
        env::var("SOCKET_ADDRESS").expect("SOCKET_ADDRESS is not set in .env file");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(Files::new(
                "/static",
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/static"),
            ))
            .service(www::trails_controller::show)
            .service(www::trails_controller::index)
  //          .service(www::teams_controller::index)
  //          .service(www::miscellaneous_controller::home)
  //          .service(
  //              web::scope("/service_api/v1").service(service_api::v1::games_controller::index),
  //          )
    })
    .bind(socket_address)?
    .run()
    .await
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{body::Body, test, App};
    use serde_json::json;

    #[actix_rt::test]
    async fn test_json_get() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();
        let mut app = test::init_service(App::new().data(tera).service(show)).await;
        let req = test::TestRequest::with_uri("/show.json").to_request();
        let mut resp = test::call_service(&mut app, req).await;
        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        assert!(resp.status().is_success());
        assert_eq!(
            &Body::from(json!({"follower_name":"Jill", "followee_name": "Jim" })), // or serde.....
            body
        );
    }
    #[actix_rt::test]
    #[should_panic]
    /// Template doesnt exist
    async fn test_web_get() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();
        let mut app = test::init_service(App::new().data(tera).service(show)).await;
        let req = test::TestRequest::with_uri("/show").to_request();
        let _resp = test::call_service(&mut app, req).await;
    }
}
*/
