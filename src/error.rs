use cdumay_core::{define_errors, define_kinds};

define_kinds! {
    GenericContextError = (500, "Generic context error"),
}

define_errors! {
    UnExpectedError = GenericContextError
}
