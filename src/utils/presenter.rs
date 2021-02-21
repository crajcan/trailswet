use actix_http::{Error, Response};
use actix_web::*;
use futures_util::future::{err, ok, Ready};
use serde::Serialize;
use tera::Context;

pub struct Presenter<R: Serialize, T: Into<String>> {
    pub resource: R,
    pub template: T,
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
