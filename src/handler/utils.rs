use crate::model::errors::{ApiResult, ApiError};
use actix_web::{Either, HttpResponse};
use actix_web::web::Json;
use actix_http::http::StatusCode;
use std::error::Error;
use failure::Fail;
use validator::Validate;

pub type RestApiResult<T> = Either<Json<T>, HttpResponse>;

pub fn to_json<T>(result: ApiResult<T>) -> RestApiResult<T> {
    match result {
        Ok(value) => Either::A(Json(value)),
        Err(error) => Either::B(error_body(error))
    }
}

pub fn validate_request<T: Validate>(request_value: &T) -> Option<ApiError> {
    match request_value.validate() {
        Ok(_) => None,
        Err(validation_errors) => Some(ApiError::from(validation_errors)),
    }
}

fn error_body(error: ApiError) -> HttpResponse {
    let body = match serde_json::to_string(&error) {
        Ok(body) => body,
        Err(e) => e.description().to_string(),
    };

    let mut buff = error.to_string();
    if error.print_backtrace() {
        buff += &"\n".to_string();
        buff += &error.backtrace().unwrap().to_string();
    }
    error!("{}", buff);
    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        .content_type("application/json")
        .body(body)
}