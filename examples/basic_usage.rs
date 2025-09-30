use photosi_exceptions_rust::{
    BaseError, Code, Level, ObjectNotFoundException, SecurityException, ValidationException,
    from_pms_response,
};

fn main() {
    println!("=== Photosi Exceptions Rust - Examples ===\n");

    // Example 1: Creating a simple exception
    println!("1. Creating ObjectNotFoundException:");
    let exception = ObjectNotFoundException::new(
        "User with ID 123 not found",
        Some("user_id: 123".to_string()),
    );
    println!("   Code: {}", exception.code());
    println!("   Level: {:?}", exception.level());
    println!("   Message: {}", exception);
    println!("   Detail: {:?}\n", exception.detail());

    // Example 2: Creating exception with custom level
    println!("2. Creating SecurityException with custom level:");
    let security_exception = SecurityException::with_level(
        "Unauthorized access attempt",
        Some("Missing API key".to_string()),
        Level::Fatal,
    );
    println!("   Code: {}", security_exception.code());
    println!("   Level: {:?}", security_exception.level());
    println!("   Message: {}\n", security_exception);

    // Example 3: Converting to PMS response
    println!("3. Converting to PMS response format:");
    let validation_exception = ValidationException::new(
        "Invalid email format",
        Some("email: test@invalid".to_string()),
    );
    let pms_response = validation_exception.to_pms_response();
    let json = serde_json::to_string_pretty(&pms_response).unwrap();
    println!("   JSON: {}\n", json);

    // Example 4: Creating exception from PMS response
    println!("4. Creating exception from PMS response:");
    let exception_from_pms = from_pms_response(
        Code::TIMEOUT,
        "Request timed out after 30s",
        Some("endpoint: /api/users".to_string()),
    );
    println!("   Code: {}", exception_from_pms.code());
    println!("   Message: {}", exception_from_pms);
    println!("   Detail: {:?}\n", exception_from_pms.detail());

    // Example 5: Using in a Result
    println!("5. Using exceptions in Result types:");
    match find_user(0) {
        Ok(_) => println!("   User found"),
        Err(e) => {
            println!("   Error occurred:");
            println!("   - Code: {}", e.code());
            println!("   - Level: {:?}", e.level());
            println!("   - Message: {}", e);
        }
    }
}

// Helper function demonstrating Result usage
fn find_user(id: u32) -> Result<String, Box<dyn BaseError>> {
    if id == 0 {
        return Err(Box::new(ObjectNotFoundException::new(
            format!("User with id {} not found", id),
            Some(format!("user_id: {}", id)),
        )));
    }
    Ok(format!("User {}", id))
}
