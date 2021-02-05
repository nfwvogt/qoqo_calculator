// Copyright © 2020-2021 HQS Quantum Simulations GmbH. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except
// in compliance with the License. You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software distributed under the
// License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing permissions and
// limitations underthe License.

//! calculator_float module
//!
//! Provides CalculatorFloat enum and methods for parsing and evaluating
//! mathematical expressions in string form to float

use serde::de::{Deserializer, Error, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::ops;
use std::str::FromStr;

use crate::CalculatorError;

static ATOL: f64 = f64::EPSILON;
static RTOL: f64 = 1e-8;
/// Enum combining Float and String
///
/// # Variants
///
/// * `Float` - f64 value
/// * `Str` - String instance
#[derive(Debug, Clone, PartialEq)]
pub enum CalculatorFloat {
    /// Floating point value
    Float(f64),
    /// Symbolic expression in String form
    Str(String),
}

// Implementing serde serialisation
// writing directly to string or f64
impl Serialize for CalculatorFloat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CalculatorFloat::Float(x) => serializer.serialize_f64(*x),
            CalculatorFloat::Str(x) => serializer.serialize_str(x),
        }
    }
}

// Deserializing directly from string or f64
impl<'de> Deserialize<'de> for CalculatorFloat {
    fn deserialize<D>(deserializer: D) -> Result<CalculatorFloat, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TemporaryVisitor;
        impl<'de> Visitor<'de> for TemporaryVisitor {
            type Value = CalculatorFloat;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("float or string")
            }

            fn visit_str<E>(self, value: &str) -> Result<CalculatorFloat, E>
            where
                E: Error,
            {
                Ok(CalculatorFloat::from(value))
            }

            fn visit_f64<E>(self, value: f64) -> Result<CalculatorFloat, E>
            where
                E: Error,
            {
                Ok(CalculatorFloat::from(value))
            }

            fn visit_i32<E>(self, value: i32) -> Result<CalculatorFloat, E>
            where
                E: Error,
            {
                Ok(CalculatorFloat::from(value))
            }
            fn visit_u32<E>(self, value: u32) -> Result<CalculatorFloat, E>
            where
                E: Error,
            {
                Ok(CalculatorFloat::from(value))
            }
        }

        deserializer.deserialize_any(TemporaryVisitor)
    }
}

/// Initialize CalculatorFloat from i32 value.
///
/// # Returns
///
/// * [CalculatorFloat::Float]
///
impl From<i32> for CalculatorFloat {
    fn from(item: i32) -> Self {
        CalculatorFloat::Float(item as f64)
    }
}

/// Initialize CalculatorFloat from usize value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<u32> for CalculatorFloat {
    fn from(item: u32) -> Self {
        CalculatorFloat::Float(item as f64)
    }
}

/// Initialize CalculatorFloat from i32 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a i32> for CalculatorFloat {
    fn from(item: &'a i32) -> Self {
        CalculatorFloat::Float(*item as f64)
    }
}

/// Initialize CalculatorFloat from usize reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a u32> for CalculatorFloat {
    fn from(item: &'a u32) -> Self {
        CalculatorFloat::Float(*item as f64)
    }
}

/// Initialize CalculatorFloat from f64 value.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<f64> for CalculatorFloat {
    fn from(item: f64) -> Self {
        CalculatorFloat::Float(item)
    }
}

/// Initialize CalculatorFloat from f64 reference &.
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl<'a> From<&'a f64> for CalculatorFloat {
    fn from(item: &'a f64) -> Self {
        CalculatorFloat::Float(*item)
    }
}

/// Initialize CalculatorFloat from string value.
///
/// # Returns
///
/// * `CalculatorFloat::Str`
///
impl From<String> for CalculatorFloat {
    fn from(item: String) -> Self {
        let f = f64::from_str(item.as_str());
        match f {
            Err(_) => CalculatorFloat::Str(item),
            Ok(x) => CalculatorFloat::Float(x),
        }
    }
}

/// Initialize CalculatorFloat from string reference &String
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<&String> for CalculatorFloat {
    fn from(item: &String) -> Self {
        let f = f64::from_str(item.as_str());
        match f {
            Err(_) => CalculatorFloat::Str(item.clone()),
            Ok(x) => CalculatorFloat::Float(x),
        }
    }
}

/// Initialize CalculatorFloat from str reference &str
///
/// # Returns
///
/// * `CalculatorFloat::Float`
///
impl From<&str> for CalculatorFloat {
    fn from(item: &str) -> Self {
        let f = f64::from_str(item);
        match f {
            Err(_) => CalculatorFloat::Str(String::from(item)),
            Ok(x) => CalculatorFloat::Float(x),
        }
    }
}

/// Try turning CalculatorFloat into f64 float
///
/// # Returns
///
/// * `f64`
///
/// # Panics
///
/// Panics when CalculatorFloat contains symbolic string value
///
impl TryFrom<CalculatorFloat> for f64 {
    type Error = CalculatorError;

    fn try_from(value: CalculatorFloat) -> Result<Self, Self::Error> {
        match value {
            CalculatorFloat::Float(x) => Ok(x),
            CalculatorFloat::Str(x) => Err(CalculatorError::FloatSymbolicNotConvertable { val: x }),
        }
    }
}

/// Return CalculatorFloat as String.
///
/// # Returns
///
/// * `String`
///
impl From<CalculatorFloat> for String {
    fn from(value: CalculatorFloat) -> Self {
        format!("{}", value)
    }
}

/// Initialize CalculatorFloat from CalculatorFloat reference &CalculatorFloat
///
/// # Returns
///
/// * `CalculatorFloat`
///
impl<'a> From<&'a CalculatorFloat> for CalculatorFloat {
    fn from(item: &'a CalculatorFloat) -> Self {
        (*item).clone()
    }
}

/// Implement Display trait for CalculatorFloat
///
/// Allows use of simple text formating
///
impl fmt::Display for CalculatorFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorFloat::Float(x) => write!(f, "{:e}", x),
            CalculatorFloat::Str(y) => write!(f, "{}", y),
        }
    }
}

impl CalculatorFloat {
    /// Return true when CalculatorFloat contains symbolic expression.
    pub fn is_float(&self) -> bool {
        match self {
            CalculatorFloat::Float(_) => true,
            CalculatorFloat::Str(_) => false,
        }
    }
    /// Return square root of CalculatorFloat.
    pub fn sqrt(&self) -> CalculatorFloat {
        match self {
            CalculatorFloat::Float(f) => CalculatorFloat::Float(f.sqrt()),
            CalculatorFloat::Str(s) => CalculatorFloat::Str(format!("sqrt({})", s)),
        }
    }
    /// Returns atan2 for CalculatorFloat and generic type `T`.
    ///
    /// # Arguments
    ///
    /// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
    ///
    pub fn atan2<T>(&self, other: T) -> CalculatorFloat
    where
        CalculatorFloat: From<T>,
    {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => CalculatorFloat::Float(x.atan2(y)),
                Self::Str(y) => Self::Str(format!("atan2({:e}, {})", x, &y)),
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => Self::Str(format!("atan2({}, {:e})", x, y)),
                Self::Str(y) => Self::Str(format!("atan2({}, {})", x, &y)),
            },
        }
    }

    /// Returns Power for CalculatorFloat and generic type `T` that can be cast to CalculatorFloat.
    ///
    /// # Arguments
    ///
    /// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
    ///
    pub fn powf<T>(&self, other: T) -> CalculatorFloat
    where
        CalculatorFloat: From<T>,
    {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => CalculatorFloat::Float(x.powf(y)),
                Self::Str(y) => Self::Str(format!("({:e} ^ {})", x, &y)),
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => Self::Str(format!("({} ^ {:e})", x, y)),
                Self::Str(y) => Self::Str(format!("({} ^ {})", x, &y)),
            },
        }
    }

    /// Returns exponential function exp(x) for CalculatorFloat.
    pub fn exp(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.exp()),
            Self::Str(y) => Self::Str(format!("exp({})", y)),
        }
    }
    /// Returns sine function sin(x) for CalculatorFloat.
    pub fn sin(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.sin()),
            Self::Str(y) => Self::Str(format!("sin({})", y)),
        }
    }
    /// Returns cosine function cos(x) for CalculatorFloat.
    pub fn cos(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.cos()),
            Self::Str(y) => Self::Str(format!("cos({})", y)),
        }
    }
    /// Returns arccosine function acos(x) for CalculatorFloat.
    pub fn acos(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.acos()),
            Self::Str(y) => Self::Str(format!("acos({})", y)),
        }
    }
    /// Returns absolute value abs(x) for CalculatorFloat.
    pub fn abs(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.abs()),
            Self::Str(y) => Self::Str(format!("abs({})", y)),
        }
    }
    /// Returns signum value sign(x) for CalculatorFloat.
    pub fn signum(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => CalculatorFloat::Float(x.signum()),
            Self::Str(y) => Self::Str(format!("sign({})", y)),
        }
    }
    /// Returns true if self is close to other value.
    pub fn isclose<T>(&self, other: T) -> bool
    where
        CalculatorFloat: From<T>,
    {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => (x - y).abs() <= (ATOL + RTOL * y.abs()),
                Self::Str(y) => format!("{:e}", x) == y,
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => x == &format!("{:e}", y),
                Self::Str(y) => x == &y,
            },
        }
    }
}
/// Implement `+` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Add<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => CalculatorFloat::Float(x + y),
                Self::Str(y) => {
                    if x != 0.0 {
                        Self::Str(format!("({:e} + {})", x, &y))
                    } else {
                        Self::Str(y)
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    if y != 0.0 {
                        Self::Str(format!("({} + {:e})", &x, y))
                    } else {
                        Self::Str(x)
                    }
                }
                Self::Str(y) => Self::Str(format!("({} + {})", &x, &y)),
            },
        }
    }
}

/// Implement `+=` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::AddAssign<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    fn add_assign(&mut self, other: T) {
        let other_from = CalculatorFloat::from(other);

        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = Self::Float(*x + y);
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() > ATOL {
                            Self::Str(format!("({:e} + {})", x, &y))
                        } else {
                            Self::Str(y)
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y != 0.0 {
                            Self::Str(format!("({} + {:e})", x, y))
                        } else {
                            Self::Str(x.to_owned())
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({} + {})", x, &y)),
            },
        }
    }
}

/// Implement `+` for &CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<'a, T> ops::Add<T> for &'a CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    type Output = CalculatorFloat;
    fn add(self, other: T) -> CalculatorFloat {
        let other_from = CalculatorFloat::from(other);
        match self {
            CalculatorFloat::Float(x) => match other_from {
                CalculatorFloat::Float(y) => CalculatorFloat::Float(x + y),
                CalculatorFloat::Str(y) => {
                    if (x - 0.0).abs() > ATOL {
                        CalculatorFloat::Str(format!("({:e} + {})", x, &y))
                    } else {
                        CalculatorFloat::Str(y)
                    }
                }
            },
            CalculatorFloat::Str(x) => match other_from {
                CalculatorFloat::Float(y) => {
                    if y != 0.0 {
                        CalculatorFloat::Str(format!("({} + {:e})", x, y))
                    } else {
                        CalculatorFloat::Str(x.to_owned())
                    }
                }
                CalculatorFloat::Str(y) => CalculatorFloat::Str(format!("({} + {})", x, &y)),
            },
        }
    }
}

/// Implement `/` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
/// # Panics
///
/// Panics on division by zero.
/// Division by zero is only detected when other is converted to CalculatorFloat::Float
///
impl<T> ops::Div<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    if y == 0.0 {
                        panic!("Division by zero")
                    } else {
                        Self::Float(x / y)
                    }
                }
                Self::Str(y) => {
                    if x == 0.0 {
                        Self::Float(0.0)
                    } else {
                        Self::Str(format!("({:e} / {})", x, &y))
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    if y == 0.0 {
                        panic!("Division by zero")
                    } else if (y - 1.0).abs() < ATOL {
                        Self::Str(x)
                    } else {
                        Self::Str(format!("({} / {:e})", &x, y))
                    }
                }
                Self::Str(y) => Self::Str(format!("({} / {})", &x, &y)),
            },
        }
    }
}

/// Implement `/=` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
/// # Panics
///
/// Panics on division by zero.
/// Division by zero is only detected when other is converted to CalculatorFloat::Float
///
impl<T> ops::DivAssign<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    fn div_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y == 0.0 {
                            panic!("Division by zero")
                        } else {
                            Self::Float(*x / y)
                        }
                    }
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() < ATOL {
                            Self::Float(0.0)
                        } else {
                            Self::Str(format!("({:e} / {})", x, &y))
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y == 0.0 {
                            panic!("Division by zero")
                        } else if (y - 1.0).abs() < ATOL {
                            Self::Str(x.to_owned())
                        } else {
                            Self::Str(format!("({} / {:e})", x, y))
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({} / {})", x, &y)),
            },
        }
    }
}

/// Implement Inverse `1/x` for CalculatorFloat.
///
impl CalculatorFloat {
    pub fn recip(&self) -> CalculatorFloat {
        match self {
            Self::Float(x) => Self::Float(x.recip()),
            Self::Str(y) => Self::Str(format!("(1 / {})", y)),
        }
    }
}

/// Implement `*` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Mul<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    type Output = Self;
    fn mul(self, other: T) -> Self {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => Self::Float(x * y),
                Self::Str(y) => {
                    if x == 0.0 {
                        Self::Float(0.0)
                    } else if (x - 1.0).abs() < ATOL {
                        Self::Str(y)
                    } else {
                        Self::Str(format!("({:e} * {})", x, &y))
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    if y == 0.0 {
                        Self::Float(0.0)
                    } else if (y - 1.0).abs() < ATOL {
                        Self::Str(x)
                    } else {
                        Self::Str(format!("({} * {:e})", &x, y))
                    }
                }
                Self::Str(y) => Self::Str(format!("({} * {})", x, y)),
            },
        }
    }
}

/// Implement `*=` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::MulAssign<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    fn mul_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = Self::Float(*x * y);
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() < ATOL {
                            Self::Float(0.0)
                        } else if (*x - 1.0).abs() < ATOL {
                            Self::Str(y)
                        } else {
                            Self::Str(format!("({:e} * {})", x, y))
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y == 0.0 {
                            Self::Float(0.0)
                        } else if (y - 1.0).abs() < ATOL {
                            Self::Str(x.to_string())
                        } else {
                            Self::Str(format!("({} * {:e})", x, y))
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({} * {})", x, y)),
            },
        }
    }
}

/// Implement `-` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::Sub<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    type Output = Self;
    fn sub(self, other: T) -> Self {
        let other_from = Self::from(other);
        match self {
            CalculatorFloat::Float(x) => match other_from {
                CalculatorFloat::Float(y) => CalculatorFloat::Float(x - y),
                CalculatorFloat::Str(y) => {
                    if x != 0.0 {
                        CalculatorFloat::Str(format!("({:e} - {})", x, y))
                    } else {
                        CalculatorFloat::Str(format!("(-{})", &y))
                    }
                }
            },
            CalculatorFloat::Str(x) => match other_from {
                CalculatorFloat::Float(y) => {
                    if y != 0.0 {
                        CalculatorFloat::Str(format!("({} - {:e})", x, y))
                    } else {
                        CalculatorFloat::Str(x)
                    }
                }
                CalculatorFloat::Str(y) => CalculatorFloat::Str(format!("({} - {})", x, y)),
            },
        }
    }
}

/// Implement `-=` for CalculatorFloat and generic type `T`.
///
/// # Arguments
///
/// 1. `other` - Any type T for which CalculatorFloat::From<T> trait is implemented
///
impl<T> ops::SubAssign<T> for CalculatorFloat
where
    CalculatorFloat: From<T>,
{
    fn sub_assign(&mut self, other: T) {
        let other_from = Self::from(other);
        match self {
            Self::Float(x) => match other_from {
                Self::Float(y) => {
                    *self = Self::Float(*x - y);
                }
                Self::Str(y) => {
                    *self = {
                        if (*x - 0.0).abs() > ATOL {
                            Self::Str(format!("({:e} - {})", x, y))
                        } else {
                            Self::Str(format!("(-{})", y))
                        }
                    }
                }
            },
            Self::Str(x) => match other_from {
                Self::Float(y) => {
                    *self = {
                        if y != 0.0 {
                            Self::Str(format!("({} - {:e})", x, y))
                        } else {
                            Self::Str(x.to_owned())
                        }
                    }
                }
                Self::Str(y) => *self = Self::Str(format!("({} - {})", x, y)),
            },
        }
    }
}

/// Implement minus sign for CalculatorFloat.
impl ops::Neg for CalculatorFloat {
    type Output = CalculatorFloat;

    fn neg(self) -> Self {
        match self {
            Self::Float(x) => Self::Float(-x),
            Self::Str(y) => Self::Str(format!("(-{})", y)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CalculatorFloat;
    use serde_test::{assert_tokens, Token};
    use std::convert::TryFrom;

    #[test]
    fn ser_de_string() {
        let x = CalculatorFloat::from("test+(1/3)");
        assert_tokens(&x, &[Token::String("test+(1/3)")]);
    }

    #[test]
    fn ser_de_float() {
        let x = CalculatorFloat::from(3.0);
        assert_tokens(&x, &[Token::F64(3.0)]);
    }

    #[test]
    fn ser_de_int() {
        let x = CalculatorFloat::from(0);
        assert_tokens(&x, &[Token::F64(0.0)]);
    }

    #[test]
    fn from() {
        // Float init
        let x = CalculatorFloat::from(3);
        if let CalculatorFloat::Float(y) = x {
            assert!((y - 3.0).abs() < f64::EPSILON)
        }
        assert!(x.is_float());
        // String init
        let x = CalculatorFloat::from("3t");
        if let CalculatorFloat::Str(y) = x.clone() {
            assert_eq!(y, "3t")
        }
        assert!(!x.is_float());
        let mut test_string = String::from("3t");
        let x = CalculatorFloat::from(&test_string);
        test_string.push_str(&String::from("2t"));
        if let CalculatorFloat::Str(y) = x.clone() {
            assert_eq!(y, "3t")
        }
        assert!(!x.is_float());
    }

    #[test]
    #[should_panic]
    fn fail_try_from() {
        let x2 = CalculatorFloat::from("test");
        f64::try_from(x2).unwrap();
    }

    #[test]
    fn try_from() {
        let x2 = CalculatorFloat::from("2");
        let x: f64 = 2.0;
        assert!((x - f64::try_from(x2).unwrap()).abs() < f64::EPSILON);
        let x3 = CalculatorFloat::from(2.0);
        assert!((x - f64::try_from(x3).unwrap()).abs() < f64::EPSILON);
    }

    #[test]
    fn add() {
        // Float init
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(2.0);
        if let CalculatorFloat::Float(y) = x3.clone() + x2.clone() {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }
        if let CalculatorFloat::Float(y) = x3.clone() + 2 {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }
        if let CalculatorFloat::Float(y) = x3.clone() + 2.0 {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }

        x3 += x2.clone();
        if let CalculatorFloat::Float(y) = x3.clone() {
            assert!((y - 5.0).abs() < f64::EPSILON)
        }
        let mut x3s = CalculatorFloat::from("3t");
        if let CalculatorFloat::Str(y) = x3s.clone() + x2.clone() {
            assert_eq!(y, "(3t + 2e0)")
        }
        if let CalculatorFloat::Str(y) = x3s.clone() + "2e0" {
            assert_eq!(y, "(3t + 2e0)")
        }
        if let CalculatorFloat::Str(y) = x3s.clone() + x2.clone() {
            assert_eq!(y, "(3t + 2e0)")
        }

        x3s += x2;
        if let CalculatorFloat::Str(y) = x3s {
            assert_eq!(y, "(3t + 2e0)")
        }
    }

    #[test]
    fn div() {
        // Float init
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(x3.clone() / x2.clone(), CalculatorFloat::Float(1.0));
        assert_eq!(x3.clone() / 3, CalculatorFloat::Float(1.0));
        assert_eq!(x3.clone() / 3.0, CalculatorFloat::Float(1.0));

        x3 /= x2.clone();
        assert_eq!(x3, CalculatorFloat::Float(1.0));
        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() / x2.clone(),
            CalculatorFloat::Str(String::from("(3t / 3e0)"))
        );
        assert_eq!(
            x3s.clone() / 2.0,
            CalculatorFloat::Str(String::from("(3t / 2e0)"))
        );
        assert_eq!(
            x3s.clone() / 2.0,
            CalculatorFloat::Str(String::from("(3t / 2e0)"))
        );
        assert_eq!(
            x3s.clone() / "2.0",
            CalculatorFloat::Str(String::from("(3t / 2e0)"))
        );
        x3s /= x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t / 3e0)")));
    }

    #[test]
    fn sub() {
        // Float init
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(x3.clone() - x2.clone(), CalculatorFloat::Float(0.0));
        assert_eq!(x3.clone() - 3, CalculatorFloat::Float(0.0));
        assert_eq!(x3.clone() - 3.0, CalculatorFloat::Float(0.0));

        x3 -= x2.clone();
        assert_eq!(x3, CalculatorFloat::Float(0.0));
        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() - x2.clone(),
            CalculatorFloat::Str(String::from("(3t - 3e0)"))
        );
        assert_eq!(
            x3s.clone() - 2.0,
            CalculatorFloat::Str(String::from("(3t - 2e0)"))
        );
        assert_eq!(
            x3s.clone() - 2.0,
            CalculatorFloat::Str(String::from("(3t - 2e0)"))
        );
        assert_eq!(
            x3s.clone() - "2.0",
            CalculatorFloat::Str(String::from("(3t - 2e0)"))
        );
        x3s -= x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t - 3e0)")));
    }

    #[test]
    fn mult() {
        // Float init
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(3.0);
        assert_eq!(x3.clone() * x2.clone(), CalculatorFloat::Float(9.0));
        assert_eq!(x3.clone() * 3, CalculatorFloat::Float(9.0));
        assert_eq!(x3.clone() * 3.0, CalculatorFloat::Float(9.0));

        x3 *= x2.clone();
        assert_eq!(x3, CalculatorFloat::Float(9.0));
        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() * x2.clone(),
            CalculatorFloat::Str(String::from("(3t * 3e0)"))
        );
        assert_eq!(
            x3s.clone() * 2.0,
            CalculatorFloat::Str(String::from("(3t * 2e0)"))
        );
        assert_eq!(
            x3s.clone() * 2.0,
            CalculatorFloat::Str(String::from("(3t * 2e0)"))
        );
        assert_eq!(
            x3s.clone() * "2.0",
            CalculatorFloat::Str(String::from("(3t * 2e0)"))
        );
        x3s *= x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t * 3e0)")));
    }

    #[test]
    fn neg() {
        // Float init
        let x3 = CalculatorFloat::from(3);
        let x2 = -x3.clone();
        assert_eq!(x2, CalculatorFloat::Float(-3.0));
        let x3s = CalculatorFloat::from("3t");
        let x2 = -x3s.clone();
        assert_eq!(x2, CalculatorFloat::Str(String::from("(-3t)")));
    }

    #[test]
    fn sqrt() {
        // Test sqrt
        let x3 = CalculatorFloat::from(3);
        let x2: f64 = 3.0;
        assert_eq!(CalculatorFloat::Float(x2.sqrt()), x3.sqrt());
        let x3s = CalculatorFloat::from("3t");
        assert_eq!(x3s.sqrt(), CalculatorFloat::Str(String::from("sqrt(3t)")));
    }

    #[test]
    fn acos() {
        // Test acos
        let x3 = CalculatorFloat::from(1);
        let x2: f64 = 1.0;
        assert_eq!(CalculatorFloat::Float(x2.acos()), x3.acos());
        let x3s = CalculatorFloat::from("1t");
        assert_eq!(x3s.acos(), CalculatorFloat::Str(String::from("acos(1t)")));
    }

    #[test]
    fn exp() {
        // Test acos
        let x3 = CalculatorFloat::from(3);
        let x2: f64 = 3.0;
        assert_eq!(CalculatorFloat::Float(x2.exp()), x3.exp());
        let x3s = CalculatorFloat::from("3t");
        assert_eq!(x3s.exp(), CalculatorFloat::Str(String::from("exp(3t)")));
    }

    #[test]
    fn abs() {
        // Test acos
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.abs()), x3.abs());
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(x3s.abs(), CalculatorFloat::Str(String::from("abs(-3t)")));
    }

    #[test]
    fn cos() {
        // Test cos
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.cos()), x3.cos());
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(x3s.cos(), CalculatorFloat::Str(String::from("cos(-3t)")));
    }

    #[test]
    fn sin() {
        // Test sin
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.sin()), x3.sin());
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(x3s.sin(), CalculatorFloat::Str(String::from("sin(-3t)")));
    }

    #[test]
    fn atan2() {
        // Test atan2
        let x3 = CalculatorFloat::from(-3);
        let x2: f64 = -3.0;
        assert_eq!(CalculatorFloat::Float(x2.atan2(2.0)), x3.atan2(2.0));
        let x3s = CalculatorFloat::from("-3t");
        assert_eq!(
            x3s.atan2("test"),
            CalculatorFloat::Str(String::from("atan2(-3t, test)"))
        );
    }

    #[test]
    fn add_ref() {
        // Float init
        let mut x3 = CalculatorFloat::from(3);
        let x2 = CalculatorFloat::from(2.0);
        assert_eq!(&x3 + &x2, CalculatorFloat::Float(5.0));
        assert_eq!(&x3 + 2, CalculatorFloat::Float(5.0));
        assert_eq!(&x3 + 2.0, CalculatorFloat::Float(5.0));

        x3 += &x2;
        assert_eq!(x3, CalculatorFloat::Float(5.0));
        let mut x3s = CalculatorFloat::from("3t");
        assert_eq!(
            x3s.clone() + x2.clone(),
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        assert_eq!(
            x3s.clone() + 2.0,
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        assert_eq!(
            x3s.clone() + 2.0,
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        assert_eq!(
            x3s.clone() + "2.0",
            CalculatorFloat::Str(String::from("(3t + 2e0)"))
        );
        x3s += x2;
        assert_eq!(x3s, CalculatorFloat::Str(String::from("(3t + 2e0)")));
    }
}