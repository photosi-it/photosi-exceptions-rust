# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-09-30

### Added
- Initial release
- BaseError trait for all exceptions
- Level enum (Debug, Info, Warning, Error, Fatal)
- Exception code constants compatible with PhotosiMessaging.Exceptions
- PmsResponse struct for PMS-compatible serialization
- All standard exception types:
  - ObjectNotFoundException
  - SecurityException
  - ValidationException
  - DbRowLockedException
  - DbUpdateConcurrencyException
  - TimeoutException
  - MaxRetriesExceededException
  - OperationNotAllowedException
  - SomethingWentWrongException
- Helper function `from_pms_response` to create exceptions from PMS error responses
