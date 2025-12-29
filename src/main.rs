use std::io;
use owo_colors::OwoColorize;

use unit_helper::conversion::Conversion;
use unit_helper::temperature::{Temperature, Unit as TemperatureUnit};
use unit_helper::length::{Length, Unit as LengthUnit};

fn print_goodbye() {
    println!("\n    {}", "Thank you for using this tool! <3".bold().purple())
}

fn format_temperature_choice(unit: TemperatureUnit) -> String {
    format!("You chose {} {}", unit.name(), unit.symbol())
}

fn format_length_choice(unit: LengthUnit) -> String {
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
fn run() {
    println!(
        "{}",
        "\n------ ^^ Unit Conversion Tool ^^ ------\n"
            .bold()
            .purple()
    );

    loop {

    println!("{} Temperature", "(1)".bold().green());
    println!("{} Length", "(2)".bold().green());
    println!("{} Quit", "(q/Q)".bold().green());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => run_temperature_flow(),
            "2" => run_length_flow(),
            "q" => return,
            "Q" => return,
            _ => println!(
                "\n{}\n",
                    "Sorry, I couldn't read your input D:\nIt must've been either a string or an invalid number.\nPlease, try again ^^"
                    .bold()
                    .red()
            ),
        }
    }
}

fn run_temperature_flow() {
    conversion_flow::<Temperature, TemperatureUnit, _, _, _>(
        "Temperature Conversion Tool",
        ask_temperature_unit,
        ask_temperature_value,
        format_temperature_choice,
    );
}

fn run_length_flow() {
    conversion_flow::<Length, LengthUnit, _, _, _>(
        "Length Conversion Tool",
        ask_length_unit,
        ask_length_value,
        format_length_choice,
    );
}

// TODO: Make it generic
fn ask_temperature_unit() -> Option<TemperatureUnit> {
    loop {
        println!("{} Fahrenheit {}", "(1)".bold().green(), "°F".blue());
        println!("{} Celsius {}", "(2)".bold().green(), "°C".blue());
        println!("{} Kelvin {}", "(3)".bold().green(), "°K".blue());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Some(TemperatureUnit::Fahrenheit),
            "2" => return Some(TemperatureUnit::Celsius),
            "3" => return Some(TemperatureUnit::Kelvin),
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

fn ask_length_unit() -> Option<LengthUnit> {
    loop {
        println!("{} Meters {}", "(1)".bold().green(), "m".blue());
        println!("{} Kilometers {}", "(2)".bold().green(), "km".blue());
        println!("{} Centimeters {}", "(3)".bold().green(), "cm".blue());
        println!("{} Inches {}", "(3)".bold().green(), "in".blue());
        println!("{} Feet {}", "(3)".bold().green(), "ft".blue());

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim() {
            "1" => return Some(LengthUnit::Meters),
            "2" => return Some(LengthUnit::Kilometers),
            "3" => return Some(LengthUnit::Centimeters),
            "4" => return Some(LengthUnit::Inches),
            "5" => return Some(LengthUnit::Feet),
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
fn ask_temperature_value(u: TemperatureUnit) -> Option<Temperature> {
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

fn ask_length_value(u: LengthUnit) -> Option<Length> {
    loop {
        println!(
            "\nNow just tell me the length n.n {}",
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

        match Length::new(value, u) {
            Ok(tmp) => return Some(tmp),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        }
    }
}

fn conversion_flow<T, U, AskUnit, AskValue, PrintChoice>(
    title: &str,
    ask_unit: AskUnit,
    ask_value: AskValue,
    print_choice: PrintChoice
)
    where
        T:Conversion<Unit = U> + std::fmt::Display + Copy,
        <T as Conversion>::Error: std::fmt::Display,
        U: Copy,
        AskUnit: Fn() -> Option<U>,
        AskValue: Fn(U) -> Option<T>,
        PrintChoice: Fn(U) -> String,
{
    println!(
        "{}",
        format!("\n------ ^^ {} ^^ ------\n", title)
            .bold()
            .purple()
    );

    println!(
        "{} {} {}\n",
        "Press".bold(),
        "(q/Q)".bold().green(),
        "to exit at any moment".bold(),
    );

    let source = match quit_or_return(ask_unit()) {
        Some(u) => u,
        None => return,
    };

    println!("\n--- {} ---", print_choice(source).bold().green());

    let target = match quit_or_return(ask_unit()) {
        Some(u) => u,
        None => return,
    };

    println!("\n--- {} ---", print_choice(target).bold().green());

    let value = match ask_value(source) {
        Some(v) => v,
        None => return,
    };

    let canonical = match value.to_canonical() {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let result = match T::from_canonical(canonical, target) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("\n          {}", "Awesome!!! :D".bold().purple());
    println!(
        "\n--- {} equals to {}! ^^ ---",
        value.bold().green(),
        result.bold().green()
    );
}

fn main() {
    run();
    // if let Err(err) = run() {
    //     eprintln!("{}", err);
    // }
    print_goodbye()
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEMPERATURE
    
    #[test]
    fn from_celsius_to_kelvin_works() {
        let temp = Temperature::new(13.0, TemperatureUnit::Celsius).expect("valid celsius temperature");

        let result = temp.to_canonical().expect("conversion to kelvin should work");

        assert!((result.value() - 286.15).abs() < 0.01);
    }

    #[test]
    fn from_fahrenheit_to_kelvin_works() {
        let temp = Temperature::new(13.0, TemperatureUnit::Fahrenheit).expect("valid fahrenheit temperature");

        let result = temp.to_canonical().expect("conversion to kelvin should work");

        assert!((result.value() - 262.594).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_celsius_works() {
        let temp = Temperature::new(286.15, TemperatureUnit::Kelvin).expect("valid kelvin temperature");

        let result =
            Temperature::from_canonical(temp, TemperatureUnit::Celsius).expect("conversion to celsius should work");

        assert!((result.value() - 13.0).abs() < 0.01);
    }

    #[test]
    fn from_kelvin_to_fahrenheit_works() {
        let temp = Temperature::new(262.594, TemperatureUnit::Kelvin).expect("valid kelvin temperature");

        let result =
            Temperature::from_canonical(temp, TemperatureUnit::Fahrenheit).expect("conversion to celsius should work");

        assert!((result.value() - 13.0).abs() < 0.01);
    }

    // LENGTH

    #[test]
    fn from_kilometers_to_meters_works() {
        let length = Length::new(1.3, LengthUnit::Kilometers);

        let result = length.expect("length creation should work").to_canonical().expect("conversion to meters should work");

        assert!((result.value() - 1300.0).abs() < 0.01);
    }

    #[test]
    fn from_meters_to_kilometers_works() {
        let length = Length::new(1300.0, LengthUnit::Meters);

        let result = Length::from_canonical(length.expect("length creation should work"), LengthUnit::Kilometers).expect("conversion to kilometers should work");

        assert!((result.value() - 1.3).abs() < 0.01);
    }

    // TODO: Solve this lol

    // Generic test function
    // fn round_trip<T, U, F>(value: f64, unit: U, constructor: F)
    //     where
    //         T: Conversion + Copy,
    //         U: Copy,
    //         F: Fn(f64, U) -> T,
    // {
    //     let original = constructor(value, unit);
    //     let canon = original.to_canonical().expect("conversion to canonical should succeed");
    //     let round_trip = T::from_canonical(canon, unit).expect("conversion from canonical should succeed");
    //
    //     let epsilon = 0.01;
    //     assert!((round_trip.value - original.value).abs() < epsilon);
    // }
}
