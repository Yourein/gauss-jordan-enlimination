mod error;
mod fraction;
mod matrix;

use fraction::Fraction;
use matrix::Matrix;

#[allow(non_snake_case)]
fn main() {
    /*
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

    println!{"----------\n{A}\n----------"};
    println!{"x1 = {}, x2 = {}, x3 = {}\n", A.get(0, 3).unwrap(), A.get(1, 3).unwrap(), A.get(2, 3).unwrap()};
    */
    let mut B = Matrix::new(4, 5);
    B.put(0, 0, Fraction::from(3)).unwrap();
    B.put(0, 1, Fraction::from(7)).unwrap();
    B.put(0, 2, Fraction::from(-1)).unwrap();
    B.put(0, 3, Fraction::from(-1)).unwrap();
    B.put(0, 4, Fraction::from(-2)).unwrap();

    B.put(1, 0, Fraction::from(-2)).unwrap();
    B.put(1, 1, Fraction::from(-2)).unwrap();
    B.put(1, 2, Fraction::from(1)).unwrap();
    B.put(1, 3, Fraction::from(1)).unwrap();
    B.put(1, 4, Fraction::from(2)).unwrap();

    B.put(2, 0, Fraction::from(2)).unwrap();
    B.put(2, 1, Fraction::from(5)).unwrap();
    B.put(2, 2, Fraction::from(1)).unwrap();
    B.put(2, 3, Fraction::from(-1)).unwrap();
    B.put(2, 4, Fraction::from(0)).unwrap();

    B.put(3, 0, Fraction::from(-4)).unwrap();
    B.put(3, 1, Fraction::from(-9)).unwrap();
    B.put(3, 2, Fraction::from(3)).unwrap();
    B.put(3, 3, Fraction::from(3)).unwrap();
    B.put(3, 4, Fraction::from(6)).unwrap();

    println!{"{B}"};

    B.eliminate(true);
    println!{"----------\n{B}\n----------"};
    println!{"x1 = {}, x2 = {}, x3 = {}, x4 = {}", B.get(0, 4).unwrap(), B.get(1, 4).unwrap(), B.get(2, 4).unwrap(), B.get(3, 4).unwrap()};
}
