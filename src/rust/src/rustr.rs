use extendr_api::prelude::*;
use extendr_api::RType::String;
use polars::frame::DataFrame;
use polars::prelude::Float64Type;
use polars::prelude::Result as PolarResult;
use polars::prelude::{CsvReader, SerReader};
use smartcore::linalg::naive::dense_matrix::DenseMatrix;
use smartcore::linalg::BaseMatrix;
use smartcore::linear::linear_regression::LinearRegression;
use smartcore::metrics::mean_squared_error;
use smartcore::model_selection::train_test_split;
use std::convert::TryFrom;
use std::fs::File;
use std::path::Path;


pub fn read_csv<P: AsRef<Path>>(path: P) -> PolarResult<DataFrame> {
    let file = File::open(path).expect("Cannot open file.");
    let df = CsvReader::new(file).has_header(true).finish();
    df
}

pub fn feature_and_target(
    in_df: &DataFrame,
    target_column: &str,
) -> (PolarResult<DataFrame>, PolarResult<DataFrame>) {
    let target = in_df.select(target_column);
    let predictors = in_df.drop(target_column);
    let predictor_names = predictors.as_ref().unwrap().get_column_names();
    println!("Predicting on {:?}", target_column);
    let predictor_names = predictor_names
        .into_iter()
        .map(|i| i.to_string() + ", ")
        .collect::<std::string::String>();
    println!("...using features {:?}", predictor_names);
    (predictors, target)
}

pub fn convert_features_to_matrix(in_df: &DataFrame) -> Result<DenseMatrix<f64>> {
    let nrows = in_df.height();
    let ncols = in_df.width();

    let features_res = in_df.to_ndarray::<Float64Type>().unwrap();
    let mut xmatrix: DenseMatrix<f64> = BaseMatrix::zeros(nrows, ncols);
    let mut col: u32 = 0;
    let mut row: u32 = 0;

    for val in features_res.iter() {
        let m_row = usize::try_from(row).unwrap();
        let m_col = usize::try_from(col).unwrap();
        xmatrix.set(m_row, m_col, *val);
        if m_col == ncols - 1 {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    Ok(xmatrix)
}

/// execute_lr
/// TODO execute_lf
/// @export
#[extendr]
pub fn execute_lr() {
    let file_path = "boston_dataset.csv";
    let target = "chas";

    let df = read_csv(&file_path).unwrap();
    let (features, target) = feature_and_target(&df, target);
    let x_matrix = convert_features_to_matrix(&features.unwrap());
    let target_array = target.unwrap().to_ndarray::<Float64Type>().unwrap();

    let mut y: Vec<f64> = Vec::new();
    for val in target_array.iter() {
        y.push(*val);
    }

    let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix.unwrap(), &y, 0.3, true);

    let linear_regression = LinearRegression::fit(&x_train, &y_train, Default::default()).unwrap();
    let predictions = linear_regression.predict(&x_test).unwrap();
    let mse = mean_squared_error(&y_test, &predictions);
    println!("MSE: {:?}", mse);
}


//
/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
pub fn hello_world() -> &'static str {
    "Hello world!"
}

/// Addition of Integers
/// @export
#[extendr]
pub fn add_float(x: u32, y: u32) -> u32 {
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

/// pass_string
/// @export
#[extendr]
pub fn pass_string(text: &str) -> Robj {
    rprintln!("{:?}", text);
    text.into_robj()
}
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn hello_world;
    fn add_float;
    fn return_vec;
    fn return_obj;
    fn my_sum;
    fn pass_string;
    fn execute_lr;
}
