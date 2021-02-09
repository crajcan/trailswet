use actix_web::{error, get, web, App, Error, HttpResponse, HttpServer};
use serde::Serialize;
use tera::{Context, Tera};

#[get("/{tail:.*}")]
async fn home(path: web::Path<String>, view: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let game = GameOrchestrator {
        home_team_name: "Miami Hurricanes".into(),
        away_team_name: "Nebraska Cornhuskers".into(),
    };

    match path.as_str() {
        "" => {
            let body = view
                .render("home.html", &Context::from_serialize(&game).unwrap())
                .map_err(|_| error::ErrorInternalServerError("Template error"))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        }
        ".json" => Ok(HttpResponse::Ok().json(game)),
        _ => Err(error::ErrorNotFound("Resource Not Found")),
    }
}

#[derive(Serialize)]
struct GameOrchestrator {
    home_team_name: String,
    away_team_name: String,
}

//try foo/{bar}/{tail:.*} to match /home.json
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
