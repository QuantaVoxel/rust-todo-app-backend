use salvo::prelude::*;
use scrutiny::traits::Validate;
use serde::de::DeserializeOwned;

use crate::error::ApiError;

/// Parse request payload as JSON and validate using scrutiny
pub async fn parse_and_validate<T>(req: &mut Request) -> Result<T, ApiError>
where
    T: DeserializeOwned + Validate,
{
    let bytes = req
        .payload()
        .await
        .map_err(|e| ApiError::BadRequest(format!("Gagal membaca body request: {:?}", e)))?;

    scrutiny::deserialize::from_json::<T>(bytes).map_err(ApiError::Validation)
}
