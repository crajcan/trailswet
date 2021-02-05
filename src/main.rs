use actix_web::{error, get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use tera::Tera;

#[get("/")]
async fn home(view: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut presenter = tera::Context::new();
    presenter.insert("name", &"Carson".to_owned());

    let body = view
        .render("home.html", &presenter)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
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
