    use std::fmt;
    use owo_colors::OwoColorize;
    
    #[derive(Copy, Clone, PartialEq)]
    pub enum Unit {
        Fahrenheit,
        Celsius,
        Kelvin,
    }

    // TODO: Add logic to handle more errors besides the temperature ones
    #[derive(Debug)]
    pub enum TemperatureError {
        BelowAbsoluteZero,
    }

    #[derive(Copy, Clone)]
    pub struct Temperature {
        value: f64,
        unit: Unit,
    }

    impl Temperature {
        pub fn new(value: f64, unit: Unit) -> Result<Self, TemperatureError> {
            if unit == Unit::Kelvin && value < 0.0 {
                Err(TemperatureError::BelowAbsoluteZero)
            } else {
                Ok(Self { value, unit })
            }
        }
    }

    impl Unit {
        pub fn to_kelvin(tmp: Temperature) -> Result<Temperature, TemperatureError> {
            let value = match tmp.unit {
                Self::Celsius => tmp.value + 273.15,
                Self::Fahrenheit => (tmp.value - 32.0) * (5.0 / 9.0) + 273.15,
                Self::Kelvin => tmp.value,
            };

            Temperature::new(value, Unit::Kelvin)
        }

        pub fn from_kelvin(tmp: Temperature, target: Unit) -> Result<Temperature, TemperatureError> {
            let value = match target {
                Self::Celsius => tmp.value - 273.15,
                Self::Fahrenheit => (tmp.value - 273.15) * (9.0 / 5.0) + 32.0,
                Self::Kelvin => tmp.value,
            };

            Temperature::new(value, target)
        }

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

    // Printing formatting
    impl fmt::Display for Temperature {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:.2} {}", self.value, self.unit.symbol())
        }
    }

    // Add explanation to errors
    impl fmt::Display for TemperatureError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                TemperatureError::BelowAbsoluteZero => {
                    write!(
                    f,
                    "\n{}", "Error: Temperature cannot be lower than absolute zero (0 °K).\nPlease, try again with another one ^^".bold().red()
                )
                }
            }
        }
    }
