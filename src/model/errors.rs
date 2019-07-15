use serde::{Serialize};
use std::error::Error;
use failure::{Fail, Backtrace};
use validator::ValidationErrors;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Fail, Debug, Serialize)]
#[fail(display = "An API error occurred with code {}: {}", code, message)]
pub struct ApiError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    validation_errors: Option<ValidationErrors>,
    #[serde(skip_serializing)]
    print_backtrace: bool,
    #[serde(skip_serializing)]
    backtrace: Backtrace,
}

impl ApiError {
    pub fn new_error(code: i32, message: String, print_backtrace: bool) -> ApiError {
        ApiError {
            code,
            message,
            validation_errors: None,
            backtrace: Backtrace::new(),
            print_backtrace,
        }
    }

    pub fn print_backtrace(&self) -> bool {
        self.backtrace().is_some() && self.print_backtrace
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> ApiError {
        ApiError {
            code: 1,
            message: err.description().to_string(),
            validation_errors: None,
            backtrace: Backtrace::new(),
            print_backtrace: true,
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> ApiError {
        ApiError {
            code: 1,
            message: err.description().to_string(),
            validation_errors: Some(err),
            backtrace: Backtrace::new(),
            print_backtrace: false,
        }
    }
}