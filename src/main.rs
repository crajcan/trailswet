use actix_http::{Error, Response};
use actix_web::*;
use futures_util::future::{err, ok, Ready};
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
struct GameOrchestrator {
    home_team_name: String,
    away_team_name: String,
}

#[get("/index{tail:.*}")]
async fn index() -> impl Responder {
    let orchestrator = GameOrchestrator {
        home_team_name: "Miami Hurricanes".into(),
        away_team_name: "Nebraska Cornhuskers".into(),
    };

    Presenter {
        resource: orchestrator,
        template: "index.html",
    }
}

struct Presenter<R: Serialize, T: Into<String>> {
    resource: R,
    template: T,
}

impl<R: Serialize, T: Into<String>> Responder for Presenter<R, T> {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let mime = req.match_info().query("tail");
        let view = req.app_data::<web::Data<tera::Tera>>().unwrap();

        match mime {
            "" | "/" => {
                let body = view
                    .render(
                        &self.template.into(),
                        &Context::from_serialize(&self.resource).unwrap(),
                    )
                    .unwrap();
                ok(HttpResponse::Ok().content_type("text/html").body(body))
            }
            ".json" => ok(HttpResponse::Ok().json(&self.resource)),
            _ => err(error::ErrorNotFound("Resource Not Found")),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/views/**/*")).unwrap();
        App::new().data(tera).service(index)
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
        let mut app = test::init_service(App::new().data(tera).service(index)).await;
        let req = test::TestRequest::with_uri("/index.json").to_request();
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
        let mut app = test::init_service(App::new().data(tera).service(index)).await;
        let req = test::TestRequest::with_uri("/index").to_request();
        let _resp = test::call_service(&mut app, req).await;
    }
}
