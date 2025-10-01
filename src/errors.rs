use crate::{Code, Level, PmsResponse};
use std::error::Error;
use std::fmt;

/// Base trait for all Photosi exceptions
pub trait BaseError: Error + Send + Sync + 'static {
    /// Returns the exception code
    fn code(&self) -> &'static str;

    /// Returns the exception level
    fn level(&self) -> Level;

    /// Returns the exception detail
    fn detail(&self) -> Option<&str>;

    /// Sets the exception detail
    fn set_detail(&mut self, detail: String);

    /// Converts to PMS response format
    fn to_pms_response(&self) -> PmsResponse {
        PmsResponse::new(
            self.code().to_string(),
            self.to_string(),
            self.detail().map(|s| s.to_string()),
        )
    }
}

// Macro to reduce boilerplate for error types
macro_rules! define_exception {
    ($name:ident, $code:expr, $default_level:expr) => {
        #[derive(Debug)]
        pub struct $name {
            message: String,
            detail: Option<String>,
            level: Level,
        }

        impl $name {
            pub fn new(message: impl Into<String>, detail: Option<String>) -> Self {
                Self {
                    message: message.into(),
                    detail,
                    level: $default_level,
                }
            }

            pub fn with_level(message: impl Into<String>, detail: Option<String>, level: Level) -> Self {
                Self {
                    message: message.into(),
                    detail,
                    level,
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.message)
            }
        }

        impl Error for $name {}

        impl BaseError for $name {
            fn code(&self) -> &'static str {
                $code
            }

            fn level(&self) -> Level {
                self.level
            }

            fn detail(&self) -> Option<&str> {
                self.detail.as_deref()
            }

            fn set_detail(&mut self, detail: String) {
                self.detail = Some(detail);
            }
        }
    };
}

// Define all exception types
define_exception!(ObjectNotFoundException, Code::OBJECT_NOT_FOUND, Level::Warning);
define_exception!(SecurityException, Code::INVALID_AUTHORIZATION, Level::Error);
define_exception!(ValidationException, Code::INVALID_MESSAGE, Level::Error);
define_exception!(DbRowLockedException, Code::DATABASE_ROW_LOCKED, Level::Error);
define_exception!(DbUpdateConcurrencyException, Code::DATABASE_CONCURRENCY, Level::Error);
define_exception!(TimeoutException, Code::TIMEOUT, Level::Error);
define_exception!(MaxRetriesExceededException, Code::MAX_RETRIES_EXCEEDED, Level::Error);
define_exception!(OperationNotAllowedException, Code::OPERATION_NOT_ALLOWED, Level::Error);
define_exception!(SomethingWentWrongException, Code::SOMETHING_WENT_WRONG, Level::Error);
