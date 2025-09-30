/// Exception codes compatible with PhotosiMessaging.Exceptions
pub struct Code;

impl Code {
    /// Code related to ObjectNotFoundException
    pub const OBJECT_NOT_FOUND: &'static str = "OBJECT_NOT_FOUND";

    /// Code related to a generic Exception
    pub const SOMETHING_WENT_WRONG: &'static str = "SOMETHING_WENT_WRONG";

    /// Code related to a SecurityException
    pub const INVALID_AUTHORIZATION: &'static str = "INVALID_AUTHORIZATION";

    /// Code related to a ValidationException
    pub const INVALID_MESSAGE: &'static str = "INVALID_MESSAGE";

    /// Code related to a DbRowLockedException
    pub const DATABASE_ROW_LOCKED: &'static str = "DATABASE_ROW_LOCKED";

    /// Code related to DbUpdateConcurrencyException
    pub const DATABASE_CONCURRENCY: &'static str = "DATABASE_CONCURRENCY";

    /// Code related to TimeoutException
    pub const TIMEOUT: &'static str = "TIMEOUT";

    /// Code related to MaxRetriesExceededException
    pub const MAX_RETRIES_EXCEEDED: &'static str = "MAX_RETRIES_EXCEEDED";

    /// Code related to OperationNotAllowedException
    pub const OPERATION_NOT_ALLOWED: &'static str = "OPERATION_NOT_ALLOWED";
}
