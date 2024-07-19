use serde_json::Value;
use crate::errors::api::ApiError;

pub fn to_json(response: String) -> Result<Value, ApiError> {
    let parsed_response: Value = serde_json::from_str(&response)
        .map_err(|_| ApiError::FailedParseResponse)?;

    Ok(parsed_response)
}