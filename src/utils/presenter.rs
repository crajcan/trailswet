use actix_http::{Error, Response};
use actix_web::*;
use askama::Template;
use futures_util::future::{err, ok, Ready};
use serde::Serialize;

pub struct Presenter<R: Template + Serialize> {
    pub resource: R,
}

//plan: Presenter should be Result and this is where we will decide what to do with errors
impl<R: Template + Serialize> Responder for Presenter<R> {
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let mime = req.match_info().query("tail");

        match mime {
            "" | "/" => {
                let body = &self.resource.render().unwrap();
                ok(HttpResponse::Ok().content_type("text/html").body(body))
            }
            ".json" => ok(HttpResponse::Ok().json(&self.resource)),
            _ => err(error::ErrorNotFound("Resource Not Found")),
        }
    }
}
