# Photosi Exceptions Rust

Shared exception types for Photosi Rust services, compatible with PhotosiMessaging.Exceptions (.NET).

## Features

- Type-safe exception handling with Rust's type system
- Compatible with PMS response format
- Serializable/Deserializable with Serde
- All standard Photosi exception types
- Exception levels (Debug, Info, Warning, Error, Fatal)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
photosi-exceptions-rust = { path = "../photosi-exceptions-rust" }
```

## Usage

### Basic exception creation

```rust
use photosi_exceptions_rust::{ObjectNotFoundException, BaseError};

fn find_user(id: i32) -> Result<User, Box<dyn BaseError>> {
    if id == 0 {
        return Err(Box::new(ObjectNotFoundException::new(
            format!("User with id {} not found", id),
            Some(format!("user_id: {}", id))
        )));
    }
    Ok(user)
}
```

### Converting from PMS response

```rust
use photosi_exceptions_rust::{from_pms_response, BaseError};
use serde::Deserialize;

#[derive(Deserialize)]
struct ErrorResponse {
    exception_code: String,
    exception_message: String,
    exception_detail: String,
}

fn handle_rpc_error(error: ErrorResponse) -> Box<dyn BaseError> {
    from_pms_response(
        &error.exception_code,
        &error.exception_message,
        Some(error.exception_detail),
    )
}
```

### Converting to PMS response

```rust
use photosi_exceptions_rust::{ValidationException, BaseError};

let exception = ValidationException::new(
    "Invalid email format",
    Some("email field is required".to_string())
);

let pms_response = exception.to_pms_response();
// Serialize to JSON for sending over the wire
let json = serde_json::to_string(&pms_response)?;
```

### Using with custom levels

```rust
use photosi_exceptions_rust::{SecurityException, Level, BaseError};

let exception = SecurityException::with_level(
    "Unauthorized access",
    Some("Missing API key".to_string()),
    Level::Fatal
);
```

## Available Exception Types

- `ObjectNotFoundException` - For missing entities (Level::Warning by default)
- `SecurityException` - For authorization/authentication issues
- `ValidationException` - For input validation errors
- `DbRowLockedException` - For database row locking issues
- `DbUpdateConcurrencyException` - For database concurrency conflicts
- `TimeoutException` - For timeout scenarios
- `MaxRetriesExceededException` - For retry exhaustion
- `OperationNotAllowedException` - For disallowed operations
- `SomethingWentWrongException` - Generic fallback exception

## Exception Codes

All exception codes are defined in the `Code` struct:

- `OBJECT_NOT_FOUND`
- `SOMETHING_WENT_WRONG`
- `INVALID_AUTHORIZATION`
- `INVALID_MESSAGE`
- `DATABASE_ROW_LOCKED`
- `DATABASE_CONCURRENCY`
- `TIMEOUT`
- `MAX_RETRIES_EXCEEDED`
- `OPERATION_NOT_ALLOWED`

## Exception Levels

- `Debug` (0)
- `Info` (1)
- `Warning` (2)
- `Error` (3) - Default
- `Fatal` (4)

## Compatibility

This library is designed to be compatible with the .NET PhotosiMessaging.Exceptions package,
using the same exception codes and PMS response format.
