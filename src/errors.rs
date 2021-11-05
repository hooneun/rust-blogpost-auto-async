use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
}