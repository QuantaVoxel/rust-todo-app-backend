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

    #[error("Method not allowed: {0}")]
    MethodNotAllowed(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Payload too large: {0}")]
    PayloadTooLarge(String),

    #[error("Unsupported media type: {0}")]
    UnsupportedMediaType(String),

    #[error("Unprocessable entity: {0}")]
    UnprocessableEntity(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("Bad gateway: {0}")]
    BadGateway(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Gateway timeout: {0}")]
    GatewayTimeout(String),

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
            Self::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
            Self::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::RateLimited(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadGateway(_) => StatusCode::BAD_GATEWAY,
            Self::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::GatewayTimeout(_) => StatusCode::GATEWAY_TIMEOUT,
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
            Self::MethodNotAllowed(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "METHOD_NOT_ALLOWED".to_string(),
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
            Self::PayloadTooLarge(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "PAYLOAD_TOO_LARGE".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::UnsupportedMediaType(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "UNSUPPORTED_MEDIA_TYPE".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::UnprocessableEntity(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "UNPROCESSABLE_ENTITY".to_string(),
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
            Self::BadGateway(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "BAD_GATEWAY".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::ServiceUnavailable(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "SERVICE_UNAVAILABLE".to_string(),
                    message: msg.clone(),
                    details: None,
                },
            },
            Self::GatewayTimeout(msg) => ApiErrorResponse {
                error: ApiErrorPayload {
                    code: "GATEWAY_TIMEOUT".to_string(),
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

    pub fn from_status_code(status: StatusCode, message: Option<String>) -> Self {
        match status {
            StatusCode::BAD_REQUEST => Self::BadRequest(message.unwrap_or_else(|| "Input tidak valid".into())),
            StatusCode::UNAUTHORIZED => Self::Unauthorized(message.unwrap_or_else(|| "Token tidak ada / kedaluwarsa".into())),
            StatusCode::FORBIDDEN => Self::Forbidden(message.unwrap_or_else(|| "Tidak punya izin atas resource".into())),
            StatusCode::NOT_FOUND => Self::NotFound(message.unwrap_or_else(|| "Route tidak ditemukan".into())),
            StatusCode::METHOD_NOT_ALLOWED => Self::MethodNotAllowed(message.unwrap_or_else(|| "Metode HTTP tidak diizinkan".into())),
            StatusCode::CONFLICT => Self::Conflict(message.unwrap_or_else(|| "Terjadi konflik data".into())),
            StatusCode::PAYLOAD_TOO_LARGE => Self::PayloadTooLarge(message.unwrap_or_else(|| "Ukuran payload melebihi batas".into())),
            StatusCode::UNSUPPORTED_MEDIA_TYPE => Self::UnsupportedMediaType(message.unwrap_or_else(|| "Tipe media tidak didukung".into())),
            StatusCode::UNPROCESSABLE_ENTITY => Self::UnprocessableEntity(message.unwrap_or_else(|| "Entitas tidak dapat diproses".into())),
            StatusCode::TOO_MANY_REQUESTS => Self::RateLimited(message.unwrap_or_else(|| "Terlalu banyak request".into())),
            StatusCode::BAD_GATEWAY => Self::BadGateway(message.unwrap_or_else(|| "Bad gateway".into())),
            StatusCode::SERVICE_UNAVAILABLE => Self::ServiceUnavailable(message.unwrap_or_else(|| "Layanan tidak tersedia".into())),
            StatusCode::GATEWAY_TIMEOUT => Self::GatewayTimeout(message.unwrap_or_else(|| "Gateway timeout".into())),
            _ => Self::Internal(message.unwrap_or_else(|| "Terjadi kesalahan pada server".into())),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_status_codes_and_payloads() {
        let test_cases = vec![
            (StatusCode::BAD_REQUEST, "VALIDATION_ERROR"),
            (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            (StatusCode::FORBIDDEN, "FORBIDDEN"),
            (StatusCode::NOT_FOUND, "NOT_FOUND"),
            (StatusCode::METHOD_NOT_ALLOWED, "METHOD_NOT_ALLOWED"),
            (StatusCode::CONFLICT, "CONFLICT"),
            (StatusCode::PAYLOAD_TOO_LARGE, "PAYLOAD_TOO_LARGE"),
            (StatusCode::UNSUPPORTED_MEDIA_TYPE, "UNSUPPORTED_MEDIA_TYPE"),
            (StatusCode::UNPROCESSABLE_ENTITY, "UNPROCESSABLE_ENTITY"),
            (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED"),
            (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR"),
            (StatusCode::BAD_GATEWAY, "BAD_GATEWAY"),
            (StatusCode::SERVICE_UNAVAILABLE, "SERVICE_UNAVAILABLE"),
            (StatusCode::GATEWAY_TIMEOUT, "GATEWAY_TIMEOUT"),
        ];

        for (status, expected_code) in test_cases {
            let err = ApiError::from_status_code(status, None);
            assert_eq!(err.status_code(), status);
            let payload = err.to_response_payload();
            assert_eq!(payload.error.code, expected_code);
            assert!(!payload.error.message.is_empty());
        }
    }
}
