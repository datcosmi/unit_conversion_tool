use std::fmt;
use std::io;

use owo_colors::OwoColorize;

#[derive(Copy, Clone)]
enum Unit {
    Fahrenheit,
    Celsius,
    Kelvin,
}

#[derive(Copy, Clone)]
struct Temperature {
    value: f64,
    unit: Unit,
}

impl Temperature {
    fn new(value: f64, unit: Unit) -> Self {
        Self { value, unit }
    }
}

impl Unit {
    fn to_kelvin(tmp: Temperature) -> Temperature {
        let value = match tmp.unit {
            Self::Celsius => tmp.value + 273.15,
            Self::Fahrenheit => (tmp.value - 32.0) * (5.0 / 9.0) + 273.15,
            Self::Kelvin => tmp.value,
        };

        Temperature::new(value, Unit::Kelvin)
    }

    fn from_kelvin(tmp: Temperature, target: Unit) -> Temperature {
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

// Format to print
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.unit.symbol())
    }
}

// impl fmt::Display for Unit {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Unit::Fahrenheit => write!(f, "°F"),
//             Unit::Celsius => write!(f, "°C"),
//             Unit::Kelvin => write!(f, "°K"),
//         }
//     }
// }

fn main() {
    println!(
        "{}",
        "\n------ ^^ Temperature Conversion Tool ^^ ------\n"
            .bold()
            .purple()
    );

    let source = ask_source();

    println!(
        "\n--- {} {} {} ---",
        "You chose:".bold().blue(),
        source.name().bold().green(),
        source.symbol().bold().green()
    );

    let target = ask_target();

    println!(
        "\n--- {} {} {} ---",
        "You chose:".bold().blue(),
        target.name().bold().green(),
        target.symbol().bold().green()
    );

    let temperature = ask_temperature(source);
    let canon_unit = Unit::to_kelvin(temperature);
    let result = Unit::from_kelvin(canon_unit, target);

    println!("\n          {}", "Awesome!!! :D".bold().purple());
    println!(
        "\n--- {} equals to {}! ^^ ---",
        temperature.bold().green(),
        result.bold().green()
    );
    println!("\n    {}", "Thank you for using this tool!".bold().purple());
}

fn ask_source() -> Unit {
    loop {
        println!(
            "What will we convert today? :D {}",
            "(Type number only)".bold().blue()
        );
        println!("{} Fahrenheit {}", "(1)".bold().green(), "°F".blue());
        println!("{} Celsius {}", "(2)".bold().green(), "°C".blue());
        println!("{} Kelvin {}", "(3)".bold().green(), "°K".blue());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Unit::Fahrenheit,
            "2" => return Unit::Celsius,
            "3" => return Unit::Kelvin,
            _ => println!(
                "\n{}\n",
                    "Sorry, I couldn't read your input D:\nIt must've been either a string or an invalid number\nPlease, try again ^^"
                    .bold()
                    .red()
            ),
        }
    }
}

fn ask_target() -> Unit {
    loop {
        println!(
            "And what will we convert this into? :o {}",
            "(Type number only)".bold().blue()
        );
        println!("{} Fahrenheit {}", "(1)".bold().green(), "°F".blue());
        println!("{} Celsius {}", "(2)".bold().green(), "°C".blue());
        println!("{} Kelvin {}", "(3)".bold().green(), "°K".blue());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Unit::Fahrenheit,
            "2" => return Unit::Celsius,
            "3" => return Unit::Kelvin,
            _ => println!(
                "\n{}\n",
                    "Sorry, I couldn't read your input D:\nIt must've been either a string or an invalid number\nPlease, try again ^^"
                    .bold()
                    .red()
            ),
        }
    }
}

fn ask_temperature(u: Unit) -> Temperature {
    loop {
        println!(
            "\nNow just tell me the temperature n.n {}",
            "(You can use decimals too!)".bold().blue()
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let value = match input.trim().parse::<f64>() {
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
        return Temperature::new(value, u);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_celsius_to_kelvin_works() {
        let result = Unit::to_kelvin(Temperature::new(13.0, Unit::Celsius)).value;

        assert!((result - 286.15).abs() < 0.01);
    }

    #[test]
    fn from_fahrenheit_to_kelvin_works() {
        let result = Unit::to_kelvin(Temperature::new(13.0, Unit::Fahrenheit)).value;

        assert!((result - 262.594).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_celsius_works() {
        let result = Unit::from_kelvin(Temperature::new(286.15, Unit::Kelvin), Unit::Celsius).value;

        assert!((result - 13.0).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_fahrenheit_works() {
        let result =
            Unit::from_kelvin(Temperature::new(262.594, Unit::Kelvin), Unit::Fahrenheit).value;

        assert!((result - 13.0).abs() < 0.01);
    }
}
