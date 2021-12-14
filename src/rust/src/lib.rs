use extendr_api::prelude::*;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

/// Addition of Integers
/// @export
#[extendr]
fn add_float(x: f64, y: f64) -> f64 {
    x + y
}


// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn hello_world;
    fn add_float;
}
