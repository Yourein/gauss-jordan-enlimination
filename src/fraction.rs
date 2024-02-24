use std::fmt;
use crate::error::ConvertionError;


/// This struct represents rational number.
/// There are two parameters p and q. To represent p/q;
pub struct Fraction {
    pub p: i64,
    pub q: i64
}

// Calculation

// TODO!

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
        if self.q == 1 {
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
