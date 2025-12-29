use std::fmt;

use crate::conversion::Conversion;
use super::{TemperatureError, Unit};

#[derive(Copy, Clone)]
pub struct Temperature {
    pub(crate) value: f64,
    pub(crate) unit: Unit,
}

impl Temperature {
    pub fn new(value: f64, unit: Unit) -> Result<Self, TemperatureError> {
        if unit == Unit::Kelvin && value < 0.0 {
            Err(TemperatureError::BelowAbsoluteZero)
        } else {
            Ok(Self { value, unit })
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Conversion for Temperature {
    type Unit = Unit;
    type Error = TemperatureError;

    fn to_canonical(self) -> Result<Self, Self::Error> {
        let value = match self.unit {
            Unit::Celsius => self.value + 273.15,
            Unit::Fahrenheit => (self.value - 32.0) * (5.0 / 9.0) + 273.15,
            Unit::Kelvin => self.value,
        };

        Temperature::new(value, Unit::Kelvin)
    }

    fn from_canonical(canon: Self, target: Unit) -> Result<Self, Self::Error> {
        let value = match target {
            Unit::Celsius => canon.value - 273.15,
            Unit::Fahrenheit => (canon.value - 273.15) * (9.0 / 5.0) + 32.0,
            Unit::Kelvin => canon.value,
        };

        Temperature::new(value, target)
    }
}

// Printing formatting
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.unit.symbol())
    }
}
