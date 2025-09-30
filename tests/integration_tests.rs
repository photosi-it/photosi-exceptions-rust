use photosi_exceptions_rust::{
    BaseError, Code, Level, ObjectNotFoundException, SecurityException, ValidationException,
    TimeoutException, from_pms_response,
};

#[test]
fn test_object_not_found_exception() {
    let exception = ObjectNotFoundException::new(
        "Item not found",
        Some("item_id: 42".to_string()),
    );

    assert_eq!(exception.code(), Code::OBJECT_NOT_FOUND);
    assert_eq!(exception.level(), Level::Warning);
    assert_eq!(exception.to_string(), "Item not found");
    assert_eq!(exception.detail(), Some("item_id: 42"));
}

#[test]
fn test_security_exception() {
    let exception = SecurityException::new(
        "Access denied",
        Some("user_id: 123".to_string()),
    );

    assert_eq!(exception.code(), Code::INVALID_AUTHORIZATION);
    assert_eq!(exception.level(), Level::Error);
    assert_eq!(exception.to_string(), "Access denied");
}

#[test]
fn test_exception_with_custom_level() {
    let exception = ValidationException::with_level(
        "Invalid input",
        None,
        Level::Fatal,
    );

    assert_eq!(exception.code(), Code::INVALID_MESSAGE);
    assert_eq!(exception.level(), Level::Fatal);
}

#[test]
fn test_pms_response_conversion() {
    let exception = TimeoutException::new(
        "Request timeout",
        Some("endpoint: /api/test".to_string()),
    );

    let pms_response = exception.to_pms_response();

    assert_eq!(pms_response.exception_code, Code::TIMEOUT);
    assert_eq!(pms_response.exception_message, "Request timeout");
    assert_eq!(pms_response.exception_detail, "endpoint: /api/test");
}

#[test]
fn test_pms_response_serialization() {
    let exception = ValidationException::new(
        "Invalid email",
        Some("email field required".to_string()),
    );

    let pms_response = exception.to_pms_response();
    let json = serde_json::to_string(&pms_response).unwrap();

    assert!(json.contains("ExceptionCode"));
    assert!(json.contains("INVALID_MESSAGE"));
    assert!(json.contains("Invalid email"));
}

#[test]
fn test_from_pms_response() {
    let exception = from_pms_response(
        Code::OBJECT_NOT_FOUND,
        "User not found",
        Some("user_id: 999".to_string()),
    );

    assert_eq!(exception.code(), Code::OBJECT_NOT_FOUND);
    assert_eq!(exception.to_string(), "User not found");
    assert_eq!(exception.detail(), Some("user_id: 999"));
}

#[test]
fn test_from_pms_response_unknown_code() {
    let exception = from_pms_response(
        "UNKNOWN_CODE",
        "Unknown error",
        None,
    );

    // Should fallback to SomethingWentWrongException
    assert_eq!(exception.code(), Code::SOMETHING_WENT_WRONG);
    assert_eq!(exception.to_string(), "Unknown error");
}

#[test]
fn test_set_detail() {
    let mut exception = ObjectNotFoundException::new("Not found", None);
    
    assert_eq!(exception.detail(), None);
    
    exception.set_detail("new_detail".to_string());
    
    assert_eq!(exception.detail(), Some("new_detail"));
}

#[test]
fn test_level_default() {
    assert_eq!(Level::default(), Level::Error);
}

#[test]
fn test_pms_response_null_detail() {
    let exception = SecurityException::new("Error", None);
    let pms_response = exception.to_pms_response();
    
    // When detail is None, it should be "null" for PMS compatibility
    assert_eq!(pms_response.exception_detail, "null");
}
