use actix_web::error;
use actix_web::http::StatusCode;
use derive_more::{Display, Error};
use std::convert::From;

#[derive(Debug, Display, Error)]
pub enum TrailsWetError {
    #[display(fmt = "{:?}", message)]
    UserError { code: StatusCode, message: String },
    #[display(fmt = "An unexpected error occurred.")]
    InternalError {
        code: StatusCode,
        note: Option<String>,
    },
}

impl error::ResponseError for TrailsWetError {
    fn status_code(&self) -> StatusCode {
        match &self {
            TrailsWetError::UserError { code, .. } => *code,
            TrailsWetError::InternalError { code, .. } => *code,
        }
    }
}

impl From<sqlx::Error> for TrailsWetError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => TrailsWetError::UserError {
                code: StatusCode::NOT_FOUND,
                message: "The page you were looking for cannot be found".to_string(),
            },
            _ => TrailsWetError::InternalError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                note: None,
            },
        }
    }
}
