use actix_web::error;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};
use std::convert::From;

#[derive(Debug, Display, Error)]
pub enum NetflixError {
    #[display(fmt = "{:?}", message)]
    UserError { code: StatusCode, message: String },
    #[display(fmt = "An unexpected error occurred.")]
    InternalError {
        code: StatusCode,
        note: Option<String>,
    },
}

impl error::ResponseError for NetflixError {
    fn status_code(&self) -> StatusCode {
        match &self {
            NetflixError::UserError { code, .. } => *code,
            NetflixError::InternalError { code, .. } => *code,
        }
    }
}

impl From<sqlx::Error> for NetflixError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => NetflixError::UserError {
                code: StatusCode::NOT_FOUND,
                message: "The page you were looking for cannot be found".to_string(),
            },
            _ => NetflixError::InternalError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                note: None,
            },
        }
    }
}
