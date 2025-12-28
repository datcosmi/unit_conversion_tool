use owo_colors::OwoColorize;
use std::fmt;

#[derive(Debug)]
pub enum TemperatureError {
    BelowAbsoluteZero,
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
