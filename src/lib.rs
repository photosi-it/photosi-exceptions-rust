pub mod code;
pub mod level;
pub mod errors;
pub mod response;

pub use code::Code;
pub use level::Level;
pub use response::PmsResponse;

// Re-export all error types
pub use errors::{
    BaseError,
    ObjectNotFoundException,
    SecurityException,
    ValidationException,
    DbRowLockedException,
    DbUpdateConcurrencyException,
    TimeoutException,
    MaxRetriesExceededException,
    OperationNotAllowedException,
    SomethingWentWrongException,
};

/// Helper function to create an exception from PMS response
pub fn from_pms_response(
    code: &str,
    message: &str,
    detail: Option<String>,
) -> Box<dyn BaseError> {
    match code {
        Code::OBJECT_NOT_FOUND => Box::new(ObjectNotFoundException::new(message, detail)),
        Code::INVALID_AUTHORIZATION => Box::new(SecurityException::new(message, detail)),
        Code::INVALID_MESSAGE => Box::new(ValidationException::new(message, detail)),
        Code::DATABASE_ROW_LOCKED => Box::new(DbRowLockedException::new(message, detail)),
        Code::DATABASE_CONCURRENCY => Box::new(DbUpdateConcurrencyException::new(message, detail)),
        Code::TIMEOUT => Box::new(TimeoutException::new(message, detail)),
        Code::MAX_RETRIES_EXCEEDED => Box::new(MaxRetriesExceededException::new(message, detail)),
        Code::OPERATION_NOT_ALLOWED => Box::new(OperationNotAllowedException::new(message, detail)),
        _ => Box::new(SomethingWentWrongException::new(message, detail)),
    }
}
