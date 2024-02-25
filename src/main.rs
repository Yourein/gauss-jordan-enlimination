mod error;
mod fraction;
mod matrix;

use fraction::Fraction;
use matrix::Matrix;

#[allow(non_snake_case)]
fn main() {
    let mut A = Matrix::new(3, 4);
    A.put(0, 0, Fraction::from(8)).unwrap();
    A.put(0, 1, Fraction::from(9)).unwrap();
    A.put(0, 2, Fraction::from(10)).unwrap();
    A.put(0, 3, Fraction::from(1630)).unwrap();

    A.put(1, 0, Fraction::from(9)).unwrap();
    A.put(1, 1, Fraction::from(10)).unwrap();
    A.put(1, 2, Fraction::from(8)).unwrap();
    A.put(1, 3, Fraction::from(1630)).unwrap();

    A.put(2, 0, Fraction::from(7)).unwrap();
    A.put(2, 1, Fraction::from(9)).unwrap();
    A.put(2, 2, Fraction::from(10)).unwrap();
    A.put(2, 3, Fraction::from(1580)).unwrap();
    
    println!{"{A}"};

    A.eliminate(true);

    println!{"{A}"};
}
