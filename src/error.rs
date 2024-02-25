use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertionError {
    #[error("The fraction can not be convert to an integer. Check if the numerator is a multiple of the denominator.")]
    NotAnInteger,
}

#[derive(Error, Debug)]
pub enum CalculationError {
    #[error("The denominator is zero")]
    ZeroDenominator,
}

#[derive(Error, Debug)]
pub enum MatrixOperationError {
    #[error("Index Out of Bounds")]
    IndexOutOfBounds,
    #[error("Invalid Operation")]
    InvalidOperation,
}
