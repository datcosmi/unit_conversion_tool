use std::fmt;
use std::io;

use owo_colors::OwoColorize;

#[derive(Copy, Clone, PartialEq)]
enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

#[derive(Debug)]
enum TemperatureError {
    BelowAbsoluteZero,
}

#[derive(Copy, Clone)]
struct Temperature {
    value: f64,
    unit: Unit,
}

impl Temperature {
    fn new(value: f64, unit: Unit) -> Result<Self, TemperatureError> {
        if unit == Unit::Kelvin && value < 0.0 {
            Err(TemperatureError::BelowAbsoluteZero)
        } else {
            Ok(Self { value, unit })
        }
    }
}

impl Unit {
    fn to_kelvin(tmp: Temperature) -> Result<Temperature, TemperatureError> {
        let value = match tmp.unit {
            Self::Celsius => tmp.value + 273.15,
            Self::Fahrenheit => (tmp.value - 32.0) * (5.0 / 9.0) + 273.15,
            Self::Kelvin => tmp.value,
        };

        Temperature::new(value, Unit::Kelvin)
    }

    fn from_kelvin(tmp: Temperature, target: Unit) -> Result<Temperature, TemperatureError> {
        let value = match target {
            Self::Celsius => tmp.value - 273.15,
            Self::Fahrenheit => (tmp.value - 273.15) * (9.0 / 5.0) + 32.0,
            Self::Kelvin => tmp.value,
        };

        Temperature::new(value, target)
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Fahrenheit => "Fahrenheit",
            Self::Celsius => "Celsius",
            Self::Kelvin => "Kelvin",
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            Self::Fahrenheit => "°F",
            Self::Celsius => "°C",
            Self::Kelvin => "°K",
        }
    }
}

// Formats to print
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.unit.symbol())
    }
}

impl fmt::Display for TemperatureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureError::BelowAbsoluteZero => {
                write!(
                    f,
                    "\n{}", "Temperature cannot be lower than absolute zero (0 °K).\nPlease, try again with another one ^^".bold().red()
                )
            }
        }
    }
}

fn print_goodbye() {
    println!("\n    {}", "Thank you for using this tool! <3".bold().purple())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
    }
}

fn run() -> Result<(), TemperatureError> {
    println!(
        "{}",
        "\n------ ^^ Temperature Conversion Tool ^^ ------\n"
            .bold()
            .purple()
    );

    println!(
        "{} {} {}\n",
        "Press".bold(),
        "(q/Q)".bold().green(),
        "to exit at any moment".bold(),
    );

    println!(
        "What will we convert today? :D {}",
        "(Type number only)".bold().blue()
    );

    let source = match ask_unit() {
        Some(u) => u,
        None => {print_goodbye(); return Ok(());}
    };

    println!(
        "\n--- {} {} {} ---",
        "You chose:".bold().blue(),
        source.name().bold().green(),
        source.symbol().bold().green()
    );

    println!(
        "\nAnd what will we convert this into? :o {}",
        "(Type number only)".bold().blue()
    );

    let target = match ask_unit() {
        Some(u) => u,
        None => {print_goodbye(); return Ok(());}
    };

    println!(
        "\n--- {} {} {} ---",
        "You chose:".bold().blue(),
        target.name().bold().green(),
        target.symbol().bold().green()
    );
    
    let temperature = match ask_temperature(source) {
        Some(t) => t,
        None => {print_goodbye(); return Ok(());}
    };

    let canon_unit = Unit::to_kelvin(temperature)?;
    let result = Unit::from_kelvin(canon_unit, target)?;

    println!("\n          {}", "Awesome!!! :D".bold().purple());
    println!(
        "\n--- {} equals to {}! ^^ ---",
        temperature.bold().green(),
        result.bold().green()
    );

    print_goodbye();

    Ok(())
}

fn ask_unit() -> Option<Unit> {
    loop {
        println!("{} Fahrenheit {}", "(1)".bold().green(), "°F".blue());
        println!("{} Celsius {}", "(2)".bold().green(), "°C".blue());
        println!("{} Kelvin {}", "(3)".bold().green(), "°K".blue());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        match trimmed {
            "1" => return Some(Unit::Fahrenheit),
            "2" => return Some(Unit::Celsius),
            "3" => return Some(Unit::Kelvin),
            "q" | "Q" => return None,
            _ => println!(
                "\n{}\n",
                    "Sorry, I couldn't read your input D:\nIt must've been either a string or an invalid number.\nPlease, try again ^^"
                    .bold()
                    .red()
            ),
            
        }
    }
}

fn ask_temperature(u: Unit) -> Option<Temperature> {
    loop {
        println!(
            "\nNow just tell me the temperature n.n {}",
            "(You can use decimals too!)".bold().blue()
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        if trimmed == "q" || trimmed == "Q" {
            return None;
        }

        let value = match trimmed.parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "\n{}",
                    "Sorry, I couldn't read your input D:\nIt must've been either a string or an invalid number\nPlease, try again ^^"
                        .bold()
                        .red()
                );
                continue;
            }
        };

        match Temperature::new(value, u) {
            Ok(tmp) => return Some(tmp),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_celsius_to_kelvin_works() {
        let temp = Temperature::new(13.0, Unit::Celsius).expect("valid celsius temperature");

        let result = Unit::to_kelvin(temp).expect("conversion to kelvin should work");

        assert!((result.value - 286.15).abs() < 0.01);
    }

    #[test]
    fn from_fahrenheit_to_kelvin_works() {
        let temp = Temperature::new(13.0, Unit::Fahrenheit).expect("valid fahrenheit temperature");

        let result = Unit::to_kelvin(temp).expect("conversion to kelvin should work");

        assert!((result.value - 262.594).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_celsius_works() {
        let temp = Temperature::new(286.15, Unit::Kelvin).expect("valid kelvin temperature");

        let result =
            Unit::from_kelvin(temp, Unit::Celsius).expect("conversion to celsius should work");

        assert!((result.value - 13.0).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_fahrenheit_works() {
        let temp = Temperature::new(262.594, Unit::Kelvin).expect("valid kelvin temperature");

        let result =
            Unit::from_kelvin(temp, Unit::Fahrenheit).expect("conversion to celsius should work");

        assert!((result.value - 13.0).abs() < 0.01);
    }
}
