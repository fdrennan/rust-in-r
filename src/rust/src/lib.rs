//! TODO: Write crate docs

// #![doc(html_root_url = "https://docs.rs/{{CRATE}}/0.0.0")]
// Deny a few warnings in doctests, since rustdoc `allow`s many warnings by default

// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use extendr_api::prelude::*;

mod pipeline;

use pipeline::features::convert_features_to_matrix;
use pipeline::features::feature_and_target;
use pipeline::features::split_df;
use pipeline::io::read_csv;

use polars::prelude::Float64Type;
use polars_core::frame::DataFrame;
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use smartcore::metrics::mean_squared_error;
use smartcore::model_selection::train_test_split;

/// execute_lr
/// TODO execute_lf
/// @export
#[extendr]
pub fn execute_lr(file_path: &str, target: &str) {

    let df = read_csv(&file_path).unwrap();
    let (features, target) = feature_and_target(&df, target);
    let (x_matrix, y) = split_df(features.as_ref().unwrap(), target.as_ref().unwrap());
    let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix, &y, 0.3, true);
    let linear_regression = LinearRegression::fit(&x_train, &y_train, Default::default()).unwrap();
    let predictions = linear_regression.predict(&x_test).unwrap();
    let mse = mean_squared_error(&y_test, &predictions);
    println!("MSE: {:?}", mse);
}

// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn execute_lr;
}
