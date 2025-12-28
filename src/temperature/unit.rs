#[derive(Copy, Clone, PartialEq)]
pub enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

impl Unit {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Fahrenheit => "Fahrenheit",
            Self::Celsius => "Celsius",
            Self::Kelvin => "Kelvin",
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Fahrenheit => "°F",
            Self::Celsius => "°C",
            Self::Kelvin => "°K",
        }
    }
}
