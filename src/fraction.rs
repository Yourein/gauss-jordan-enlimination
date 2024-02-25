use std::fmt;
use gcd::Gcd;
use std::ops::{Add, Sub, Mul, Div};
use crate::error::{ConvertionError, CalculationError};


/// This struct represents rational number.
/// There are two parameters p and q. To represent p/q;
#[derive(Copy, Clone, PartialEq)]
pub struct Fraction {
    pub p: i64,
    pub q: i64
}

// Calculation

impl Fraction {
    pub fn inverse(&self) -> Result<Fraction, CalculationError>  {
        if self.p == 0 {
            Err(CalculationError::ZeroDenominator)
        }
        else {
            let gcd: i64 = (self.p.abs() as u64).gcd(self.q.try_into().unwrap()).try_into().unwrap();
            let numerator = if self.p < 0 { -self.q/gcd } else { self.q/gcd };
            let denominator = if self.p < 0 { -self.p/gcd } else { self.p/gcd };
            Ok(
                Fraction {
                    p: numerator,
                    q: denominator
                }
            )
        }
    }

    pub fn is_zero(&self) -> bool {
        self.p == 0
    }
}

impl Add for Fraction {
    type Output = Fraction;

    fn add(self, right: Fraction) -> Self::Output {
        let denominator = self.q * right.q;
        let numerator = self.p * right.q + right.p * self.q;
        let gcd = (numerator.abs() as u64).gcd(denominator.abs().try_into().unwrap());

        Fraction {
            p: numerator/(gcd as i64),
            q: denominator/(gcd as i64)
        }
    }
}

impl Mul for Fraction {
    type Output = Fraction;
    
    fn mul(self, right: Fraction) -> Self::Output {
        let numerator = self.p * right.p;
        let denominator = self.q * right.q;
        let gcd = (numerator.abs() as u64).gcd(denominator.abs().try_into().unwrap());

        Fraction {
            p: numerator/(gcd as i64),
            q: denominator/(gcd as i64)
        }
    }
}

impl Sub for Fraction {
    type Output = Fraction;

    fn sub(self, right: Fraction) -> Self::Output {
        self + Fraction::from(-1)*right
    }
}

impl Div for Fraction {
    type Output = Result<Fraction, CalculationError>;

    fn div(self, right: Fraction) -> Self::Output {
        let inv = right.inverse();
        if inv.is_err() {
            Err(inv.err().unwrap())
        }
        else {
            Ok(self * inv.unwrap())
        }
    }
}

// From other types

impl<T> From<T> for Fraction where T: Into<i64> {
    fn from(origin: T) -> Fraction {
        Fraction {
            p: origin.into(),
            q: 1
        }
    }
}

// Format

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.q == 1 || self.p == 0 {
            write!(f, "{}", self.p)
        }
        else {
            write!(f, "{}/{}", self.p, self.q)
        }
    }
}

// Into other types

impl TryInto<i32> for Fraction {
    type Error = ConvertionError;

    fn try_into(self) -> Result<i32, Self::Error> {
        if self.p % self.q != 0 {
            return Err(ConvertionError::NotAnInteger)
        }
        else {
            return Ok((self.p/self.q).try_into().unwrap())
        }
    }
}

impl TryInto<i64> for Fraction {
    type Error = ConvertionError;

    fn try_into(self) -> Result<i64, Self::Error> {
        if self.p % self.q != 0 {
            return Err(ConvertionError::NotAnInteger)
        }
        else {
            return Ok(self.p/self.q)
        }
    }
}
