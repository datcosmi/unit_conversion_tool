use std::io;
use owo_colors::OwoColorize;

mod conversion;
mod temperature;

use crate::conversion::Conversion;
use temperature::{Unit, Temperature, TemperatureError};

fn print_goodbye() {
    println!("\n    {}", "Thank you for using this tool! <3".bold().purple())
}

fn format_choice(unit: Unit) -> String {
    format!("You chose {} {}", unit.name(), unit.symbol())
}

fn quit_or_return<T>(opt: Option<T>) -> Option<T> {
    match opt {
        Some(v) => Some(v),
        None => {print_goodbye(); None}
    }
}

// TODO: Handle any kind of conversion

// Handle user errors internally and let them retry
// Only exit if irrecoverable
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

    let source = match quit_or_return(ask_unit()) {
        Some(u) => u,
        None => return Ok(()),
    };

    println!("\n--- {} ---",format_choice(source).bold().green());

    println!(
        "\nAnd what will we convert this into? :o {}",
        "(Type number only)".bold().blue()
    );

    let target = match quit_or_return(ask_unit()) {
        Some(u) => u,
        None => return Ok(()),
    };

    println!("\n--- {} ---",format_choice(target).bold().green());
    
    let temperature = match ask_temperature(source) {
        Some(t) => t,
        None => {print_goodbye(); return Ok(());}
    };

    // Convert into canon unit first, then into the desired one
    let canonical = temperature.to_canonical()?;
    let result = Temperature::from_canonical(canonical, target)?;

    println!("\n          {}", "Awesome!!! :D".bold().purple());
    println!(
        "\n--- {} equals to {}! ^^ ---",
        temperature.bold().green(),
        result.bold().green()
    );

    print_goodbye();

    Ok(()) // Return succeed to match Result
}

// TODO: Make it generic
fn ask_unit() -> Option<Unit> {
    loop {
        println!("{} Fahrenheit {}", "(1)".bold().green(), "°F".blue());
        println!("{} Celsius {}", "(2)".bold().green(), "°C".blue());
        println!("{} Kelvin {}", "(3)".bold().green(), "°K".blue());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
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

// TODO: Make it generic
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

        if trimmed == "q" || trimmed == "Q" { return None; }

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

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_celsius_to_kelvin_works() {
        let temp = Temperature::new(13.0, Unit::Celsius).expect("valid celsius temperature");

        let result = temp.to_canonical().expect("conversion to kelvin should work");

        assert!((result.value - 286.15).abs() < 0.01);
    }

    #[test]
    fn from_fahrenheit_to_kelvin_works() {
        let temp = Temperature::new(13.0, Unit::Fahrenheit).expect("valid fahrenheit temperature");

        let result = temp.to_canonical().expect("conversion to kelvin should work");

        assert!((result.value - 262.594).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_celsius_works() {
        let temp = Temperature::new(286.15, Unit::Kelvin).expect("valid kelvin temperature");

        let result =
            Temperature::from_canonical(temp, Unit::Celsius).expect("conversion to celsius should work");

        assert!((result.value - 13.0).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_fahrenheit_works() {
        let temp = Temperature::new(262.594, Unit::Kelvin).expect("valid kelvin temperature");

        let result =
            Temperature::from_canonical(temp, Unit::Fahrenheit).expect("conversion to celsius should work");

        assert!((result.value - 13.0).abs() < 0.01);
    }
}
