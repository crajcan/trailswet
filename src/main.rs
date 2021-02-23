use actix_web::*;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tera::Tera;

mod utils;
use utils::presenter::Presenter;

mod controllers;
use controllers::*;

mod orchestrators;
use orchestrators::*;

mod models;
use models::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    println!("database_url: {}", database_url);

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    println!("starting_web_server");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();

        App::new()
            .data(tera)
            .data(db_pool.clone())
            .service(games_controller::show)
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

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
