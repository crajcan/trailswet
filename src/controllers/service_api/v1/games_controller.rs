use crate::services;
use crate::{NetflixError, Presenter};
use actix_web::*;

#[get(r#"/games{tail:.*}"#)]
async fn index() -> Result<impl Responder, NetflixError> {
    Ok(Presenter {
        resource: services::v1::games::search().await?,
    })
}
