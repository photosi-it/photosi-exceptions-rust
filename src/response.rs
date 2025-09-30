use serde::{Deserialize, Serialize};

/// PMS-compatible response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmsResponse {
    #[serde(rename = "ExceptionCode")]
    pub exception_code: String,

    #[serde(rename = "ExceptionDetail")]
    pub exception_detail: String,

    #[serde(rename = "ExceptionMessage")]
    pub exception_message: String,
}

impl PmsResponse {
    pub fn new(code: String, message: String, detail: Option<String>) -> Self {
        Self {
            exception_code: code,
            exception_message: message,
            exception_detail: detail.unwrap_or_else(|| "null".to_string()),
        }
    }
}
