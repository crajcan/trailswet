use actix_service::Service;
use actix_web::{App, get, HttpServer};
use futures::future::FutureExt;

#[get("/index.html")]
async fn index() -> &'static str {
    println!("in handler");

    "Welcome!"
}


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
