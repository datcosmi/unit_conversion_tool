use std::io;
use std::fmt;

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
    println!("Convert your temperatures!");

    let conversion = ask_conversion();
    let temperature = ask_temperature();

    let result = match conversion {
        Conversion::FahrenheitToCelsius => convert_to_celsius(temperature),
        Conversion::CelsiusToFahrenheit => convert_to_fahrenheit(temperature),
    };

    println!("Result: {result}");
}

fn ask_conversion() -> Conversion {
    loop {
        println!("Choose conversion:");
        println!("(1) Fahrenheit to Celsius");
        println!("(2) Celsius to Fahrenheit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Conversion::FahrenheitToCelsius,
            "2" => return Conversion::CelsiusToFahrenheit,
            _ => println!("Invalid input. Try again \n"),
        }
    }
}

fn ask_temperature() -> f64 {
    loop {
        println!("Input the temperature value: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Try again \n"),
        }
    }
}

fn convert_to_celsius(f: f64) -> Temperature {
    let value: f64 = (f - 32.0) * 5.0 / 9.0;
    let temperature = Temperature {value: value, unit: Unit::Celsius};

    return temperature;
}

fn convert_to_fahrenheit(c: f64) -> Temperature {
    let value: f64 = (c * (9.0 / 5.0)) + 32.0;
    let temperature = Temperature {value: value, unit: Unit::Fahrenheit};

    return temperature;
}
