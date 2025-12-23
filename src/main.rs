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
    println!("{}", "\nTEMPERATURE CONVERTION HELPER\n".bold().purple());

    let conversion = ask_conversion();

    let choice = match conversion {
        Conversion::FahrenheitToCelsius => "Fahrenheit to Celsius",
        Conversion::CelsiusToFahrenheit => "Celsius to Fahrenheit",
    };

    println!("\nYou chose: {}", choice.bold().green());

    let temperature = ask_temperature();

    let result = match conversion {
        Conversion::FahrenheitToCelsius => convert_to_celsius(temperature),
        Conversion::CelsiusToFahrenheit => convert_to_fahrenheit(temperature),
    };

    let temperature = Temperature {
        value: temperature,
        unit: match conversion {
            Conversion::FahrenheitToCelsius => Unit::Fahrenheit,
            Conversion::CelsiusToFahrenheit => Unit::Celsius,
        }
    };

    println!("\n{} equals to {}", temperature.bold().green(), result.bold().green());
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
            _ => println!("\n{}\n", "Input was either a string or an invalid number. Try again.".bold().red()),
        }
    }
}

fn ask_temperature() -> f64 {
    loop {
        println!("\nInput the temperature value: {}", "(You can use decimals too!)".bold().blue());

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("\n{}", "Input was either a string or an invalid number. Try again.".bold().red()),
        }
    }
}

fn convert_to_celsius(f: f64) -> Temperature {
    let value: f64 = (f - 32.0) * 5.0 / 9.0;
    let temperature = Temperature {
        value: value,
        unit: Unit::Celsius,
    };

    return temperature;
}

fn convert_to_fahrenheit(c: f64) -> Temperature {
    let value: f64 = (c * (9.0 / 5.0)) + 32.0;
    let temperature = Temperature {
        value: value,
        unit: Unit::Fahrenheit,
    };

    return temperature;
}
