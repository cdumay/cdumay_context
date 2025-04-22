use cdumay_error::{define_errors, define_kinds, AsError};

/// Define a custom error kind for general context-related errors.
///
/// - `GenericContextError`: A unique identifier for the kind of error.
/// - Error Code: "CONTEXT-00001"
/// - HTTP Status: 500 (Internal Server Error)
/// - Description: "Generic context error"
define_kinds! {
    GenericContextError = ("CONTEXT-00001", 500, "Generic context error"),
}

/// Define a specific error type (`UnExpectedError`) that uses the previously defined `GenericContextError` kind.
/// This allows us to instantiate typed errors with a consistent structure and metadata.
define_errors! {
    UnExpectedError = GenericContextError
}
