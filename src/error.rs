use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConvertionError {
    #[error("The fraction can not be convert to an integer. Check if the numerator is a multiple of the denominator.")]
    NotAnInteger,
}
