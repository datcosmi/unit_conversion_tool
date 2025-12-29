use std::convert::Infallible;
use std::fmt;

use super::Unit;
use crate::conversion::Conversion;

#[derive(Copy, Clone)]
pub struct Length {
    pub(crate) value: f64,
    pub(crate) unit: Unit,
}

impl Length {
    pub fn new(value: f64, unit: Unit) -> Result<Self, Infallible> {
        Ok(Self { value, unit })
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

impl Conversion for Length {
    type Unit = Unit;
    type Error = Infallible;

    fn to_canonical(self) -> Result<Self, Infallible> {
        let value = match self.unit {
            Unit::Meters => self.value,
            Unit::Kilometers => self.value * 1000.0,
            Unit::Centimeters => self.value / 100.0,
            Unit::Inches => self.value * 0.0254,
            Unit::Feet => self.value * 0.3048,
        };

        Length::new(value, Unit::Meters)
    }

    fn from_canonical(canon: Self, target: Unit) -> Result<Self, Infallible> {
        let value = match target {
            Unit::Meters => canon.value,
            Unit::Kilometers => canon.value / 1000.0,
            Unit::Centimeters => canon.value * 100.0,
            Unit::Inches => canon.value / 0.0254,
            Unit::Feet => canon.value / 0.3048,
        };

        Length::new(value, target)
    }
}

// Printing formatting
impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.unit.symbol())
    }
}
