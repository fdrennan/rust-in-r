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
fn add_float(x: u32, y: u32) -> u32 {
    x + y
}

/// Return a vector
/// @export
#[extendr]
fn return_vec(x: i32, y: i32) -> Robj {
    let vec_val = r!([x, y]);
    vec_val
}

/// Return robj
/// @export
#[extendr]
fn return_obj(x: Robj) -> Robj {
    x
}

/// my_sum
/// @export
#[extendr]
pub fn my_sum(v: Robj) -> Robj {
    rprintln!("{:?}", v);
    v
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn hello_world;
    fn add_float;
    fn return_vec;
    fn return_obj;
    fn my_sum;
}
