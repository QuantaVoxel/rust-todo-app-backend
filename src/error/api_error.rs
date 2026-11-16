use salvo::http::StatusCode;
use salvo::prelude::*;
use salvo::writing::Scribe;
use scrutiny::error::ValidationErrors;
use thiserror::Error;

use crate::common::response::{ApiErrorDetail, ApiErrorPayload, ApiErrorResponse};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Validation error")]
    Validation(ValidationErrors),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Hash error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Bcrypt(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Jwt(_) => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn to_response_payload(&self) -> ApiErrorResponse {
        match self {
            Self::Validation(errors) => {
                let mut details = Vec::new();
                for (field, err_list) in errors.field_errors() {
                    for err in err_list {
                        details.push(ApiErrorDetail {
                            field: field.clone(),
                            issue: err.message.clone(),
                        });
                    }
                }
                ApiErrorResponse {
                    error: ApiErrorPayload {
                        code: "VALIDATION_ERROR".to_string(),
                        message: "Input tidak valid".to_string(),
                        details: Some(details),
                    },
                }
            }
            Self::BadRequest(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "VALIDATION_ERROR".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::Unauthorized(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "UNAUTHORIZED".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::Forbidden(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "FORBIDDEN".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::NotFound(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "NOT_FOUND".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::Conflict(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "CONFLICT".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::RateLimited(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "RATE_LIMITED".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::Internal(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "INTERNAL_ERROR".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::Database(err) => {
                tracing::error!("Database error: {:?}", err);
                ApiErrorResponse {
                    error: ApiErrorPayload {
                        code: "INTERNAL_ERROR".to_string(),
                        message: "Terjadi kesalahan pada database".to_string(),
                        details: None,
                    },
                }
            }
            Self::Bcrypt(err) => {
                tracing::error!("Bcrypt error: {:?}", err);
                ApiErrorResponse {
                    error: ApiErrorPayload {
                        code: "INTERNAL_ERROR".to_string(),
                        message: "Terjadi kesalahan saat memproses keamanan kata sandi".to_string(),
                        details: None,
                    },
                }
            }
            Self::Jwt(err) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "UNAUTHORIZED".to_string(),
                    message: format!("Token tidak valid atau kedaluwarsa: {}", err),
                    details: None,
                },
            },
        }
    }
}

impl Scribe for ApiError {
    fn render(self, res: &mut Response) {
        let status = self.status_code();
        let payload = self.to_response_payload();
        res.status_code(status);
        res.render(Json(payload));
    }
}
