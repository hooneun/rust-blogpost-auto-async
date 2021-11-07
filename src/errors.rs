use actix_web::{dev::MessageBody, error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[dispay(fmt = "JWKSFetchError")]
    JWKSFetchError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalSErverError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::JWKSFetchError => {
                HttpResponse::InternalServerError().json("Could not fetch JWKS")
            }
        }
    }
}
