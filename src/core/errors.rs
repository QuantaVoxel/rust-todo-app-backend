use salvo::http::ParseError;
use salvo::prelude::*;
use scrutiny::error::ValidationErrors;
use scrutiny::traits::Validate;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, Debug)]
pub struct ApiError {
    pub message: String,
    pub errors: BTreeMap<String, Vec<String>>,
    #[serde(skip)]
    pub status: StatusCode,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> ApiError {
        Self {
            message: message.into(),
            errors: BTreeMap::new(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn new(
        status: StatusCode,
        message: impl Into<String>,
        errors: BTreeMap<String, Vec<String>>,
    ) -> ApiError {
        Self {
            message: message.into(),
            errors,
            status,
        }
    }
}

#[async_trait]
impl Writer for ApiError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        res.status_code(self.status);
        res.render(Json(self));
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(e: ValidationErrors) -> Self {
        let mut errors = BTreeMap::new();
        let mut is_syntax_error = false;

        for (field, field_errs) in e.field_errors() {
            for err in field_errs {
                if err.rule == "deserialization" {
                    let msg = &err.message;

                    // 1. Missing field
                    if let Some(start) = msg.find("missing field `") {
                        let rest = &msg[start + "missing field `".len()..];
                        if let Some(end) = rest.find('`') {
                            let missing_field = &rest[..end];
                            errors
                                .entry(missing_field.to_string())
                                .or_insert_with(Vec::new)
                                .push("Field ini wajib diisi".to_string());
                            continue;
                        }
                    }

                    // 2. Syntax error pada JSON
                    if msg.contains("expected value")
                        || msg.contains("expected `,`")
                        || msg.contains("trailing comma")
                        || msg.contains("EOF while parsing")
                        || msg.contains("expected")
                    {
                        is_syntax_error = true;
                        errors
                            .entry("json".to_string())
                            .or_insert_with(Vec::new)
                            .push(format!("Sintaks JSON tidak valid: {msg}"));
                        continue;
                    }

                    // 3. Deserialization error lainnya (seperti invalid type)
                    let target_field = if field == "_body" { "body" } else { field };
                    errors
                        .entry(target_field.to_string())
                        .or_insert_with(Vec::new)
                        .push(msg.clone());
                } else {
                    // Error aturan validasi dari scrutiny
                    errors
                        .entry(field.clone())
                        .or_insert_with(Vec::new)
                        .push(err.message.clone());
                }
            }
        }

        if is_syntax_error && errors.contains_key("json") && errors.len() == 1 {
            Self {
                message: "Format JSON tidak valid".into(),
                errors,
                status: StatusCode::BAD_REQUEST,
            }
        } else {
            Self {
                message: "Validasi Gagal".into(),
                errors,
                status: StatusCode::UNPROCESSABLE_ENTITY,
            }
        }
    }
}

impl From<ParseError> for ApiError {
    fn from(err: ParseError) -> Self {
        let mut errors = BTreeMap::new();
        match err {
            ParseError::EmptyBody => {
                errors.insert(
                    "body".to_string(),
                    vec!["Request body tidak boleh kosong".to_string()],
                );
                Self {
                    message: "Format JSON tidak valid".into(),
                    errors,
                    status: StatusCode::BAD_REQUEST,
                }
            }
            other => {
                let msg = other.to_string();
                errors.insert("body".to_string(), vec![msg]);
                Self {
                    message: "Gagal memproses request".into(),
                    errors,
                    status: StatusCode::BAD_REQUEST,
                }
            }
        }
    }
}

pub async fn validate_json<T>(req: &mut Request) -> Result<T, ApiError>
where
    T: DeserializeOwned + Validate,
{
    let bytes = req.payload().await?;
    let data: T = scrutiny::deserialize::from_json(bytes)?;
    Ok(data)
}

pub async fn validate_query<T>(req: &mut Request) -> Result<T, ApiError>
where
    T: DeserializeOwned + Validate,
{
    let data: T = req
        .parse_queries()
        .map_err(|e| ApiError::bad_request(format!("Query String tidak valid {e}")))?;

    data.validate()?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use scrutiny::Validate;
    use serde::Deserialize;

    #[derive(Deserialize, Validate, Debug)]
    struct DummyDto {
        #[validate(email(message = "Format email tidak valid"))]
        email: String,
        #[validate(min(value = 3, message = "Password minimal 3 karakter"))]
        password: String,
    }

    #[test]
    fn test_validation_errors() {
        let dto = DummyDto {
            email: "bukan-email".into(),
            password: "ab".into(),
        };

        let err = dto.validate().unwrap_err();
        let api_err = ApiError::from(err);

        assert_eq!(api_err.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(api_err.message, "Validasi Gagal");
        assert_eq!(
            api_err.errors.get("email").unwrap(),
            &vec!["Format email tidak valid".to_string()]
        );
        assert_eq!(
            api_err.errors.get("password").unwrap(),
            &vec!["Password minimal 3 karakter".to_string()]
        );
    }

    #[test]
    fn test_json_deserialization_syntax_error() {
        let bytes = b"{\n  \"email\": \n}";
        let res = scrutiny::deserialize::from_json::<DummyDto>(bytes);
        let err = res.unwrap_err();
        let api_err = ApiError::from(err);

        assert_eq!(api_err.status, StatusCode::BAD_REQUEST);
        assert_eq!(api_err.message, "Format JSON tidak valid");
        assert!(api_err.errors.contains_key("json"));
        assert!(api_err.errors["json"][0].contains("expected value"));
    }

    #[test]
    fn test_json_missing_field() {
        let bytes = b"{\"password\": \"12345\"}";
        let res = scrutiny::deserialize::from_json::<DummyDto>(bytes);
        let err = res.unwrap_err();
        let api_err = ApiError::from(err);

        assert_eq!(api_err.status, StatusCode::UNPROCESSABLE_ENTITY);
        assert_eq!(api_err.message, "Validasi Gagal");
        assert_eq!(
            api_err.errors.get("email").unwrap(),
            &vec!["Field ini wajib diisi".to_string()]
        );
    }
}
