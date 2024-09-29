use std::io;

enum TemperatureScale {
    Fahrenheit,
    Celsius,
}

impl TemperatureScale {
    fn as_string(&self) -> String {
        match self {
            TemperatureScale::Fahrenheit => String::from("Fahrenheit"),
            TemperatureScale::Celsius => String::from("Celsius"),
        }
    }
}

fn main() {
    println!("Input C or F whether you want to start converting To Celsius or Fahrenheit scale.");
    let scale_to: TemperatureScale = get_scale();
    let scale_from = match scale_to {
        TemperatureScale::Celsius => TemperatureScale::Fahrenheit,
        TemperatureScale::Fahrenheit => TemperatureScale::Celsius,
    };

    println!(
        "Now please input value to be converted to {}",
        scale_to.as_string()
    );
    let value_from: i32 = get_scale_value();

    let value_converted = match scale_to {
        TemperatureScale::Celsius => fahrenheit_to_celsius(value_from),
        TemperatureScale::Fahrenheit => celsius_to_fahrenheit(value_from),
    };
    println!(
        "Your value of {:.1} in {} was converted to {:.1} in {}",
        value_from,
        scale_from.as_string(),
        value_converted,
        scale_to.as_string()
    )
}

fn get_scale() -> TemperatureScale {
    loop {
        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Please input C or F to begin scale conversion");

        let scale: char = match scale.trim().to_uppercase().parse() {
            Ok(character) => character,
            Err(_) => {
                println!("Please input single scale character!");
                continue;
            }
        };

        match scale {
            'C' => break TemperatureScale::Celsius,
            'F' => break TemperatureScale::Fahrenheit,
            _ => {
                println!("Please input correct scale!");
                continue;
            }
        }
    }
}

fn get_scale_value() -> i32 {
    loop {
        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Please input value to be converted");

        let value: i32 = match value.trim().parse() {
            Ok(numb) => numb,
            Err(_) => continue,
        };

        break value;
    }
}

fn fahrenheit_to_celsius(val: i32) -> f64 {
    (f64::from(val) - 32.0) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(val: i32) -> f64 {
    println!("{}", val);
    f64::from(val) * (9.0 / 5.0) + 32.0
}
