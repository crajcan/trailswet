use actix_web::error;
use derive_more::{Display, Error};
use std::convert::From;

#[derive(Debug, Display, Error)]
#[display(fmt = "my error: {}", name)]
pub struct NetflixError {
    name: String,
}

impl error::ResponseError for NetflixError {}

impl From<sqlx::Error> for NetflixError {
    fn from(e: sqlx::Error) -> Self {
        let name = e.to_string();

        NetflixError { name: name }
    }
}
