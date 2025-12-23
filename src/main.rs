use std::fmt;
use std::io;

use owo_colors::OwoColorize;

enum Conversion {
    FahrenheitToCelsius,
    CelsiusToFahrenheit,
}

enum Unit {
    Fahrenheit,
    Celsius,
}

struct Temperature {
    value: f64,
    unit: Unit,
}

impl Conversion {
    fn label(&self) -> &'static str {
        match self {
            Conversion::FahrenheitToCelsius => "Fahrenheit to Celsius",
            Conversion::CelsiusToFahrenheit => "Celsius to Fahrenheit",
        }
    }

    fn unit(&self) -> Unit {
        match self {
            Conversion::FahrenheitToCelsius => Unit::Fahrenheit,
            Conversion::CelsiusToFahrenheit => Unit::Celsius,
        }
    }

    fn apply(&self, t: f64) -> Temperature {
        match self {
            Conversion::FahrenheitToCelsius => convert_to_celsius(t),
            Conversion::CelsiusToFahrenheit => convert_to_fahrenheit(t),
        }
    }
}

// Format to print
impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {}", self.value, self.unit)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Unit::Fahrenheit => write!(f, "°F"),
            Unit::Celsius => write!(f, "°C"),
        }
    }
}

fn main() {
    println!("{}", "\nTEMPERATURE CONVERSION HELPER\n".bold().purple());

    let conversion = ask_conversion();

    println!("\nYou chose: {}", conversion.label().bold().green());

    let temperature = ask_temperature(conversion.unit());

    let result = conversion.apply(temperature.value);

    println!(
        "\n{} equals to {}",
        temperature.bold().green(),
        result.bold().green()
    );
}

fn ask_conversion() -> Conversion {
    loop {
        println!("Choose conversion: {}", "(Type number only)".bold().blue());
        println!("{} Fahrenheit to Celsius", "(1)".bold().green());
        println!("{} Celsius to Fahrenheit", "(2)".bold().green());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Conversion::FahrenheitToCelsius,
            "2" => return Conversion::CelsiusToFahrenheit,
            _ => println!(
                "\n{}\n",
                "Input was either a string or an invalid number. Try again."
                    .bold()
                    .red()
            ),
        }
    }
}

fn ask_temperature(u: Unit) -> Temperature {
    loop {
        println!(
            "\nInput the temperature value: {}",
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
                    "Input was either a string or an invalid number. Try again."
                        .bold()
                        .red()
                );
                continue;
            }
        };
        return create_temperature(value, u);
    }
}

fn convert_to_celsius(f: f64) -> Temperature {
    let value: f64 = (f - 32.0) * 5.0 / 9.0;

    return create_temperature(value, Unit::Celsius);
}

fn convert_to_fahrenheit(c: f64) -> Temperature {
    let value: f64 = (c * (9.0 / 5.0)) + 32.0;

    return create_temperature(value, Unit::Fahrenheit);
}

fn create_temperature(v: f64, u: Unit) -> Temperature {
    Temperature {
        value: v,
        unit: u,
    }
}
