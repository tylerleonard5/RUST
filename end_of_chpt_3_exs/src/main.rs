use std::io;

fn main() {
    //Converts temperatures
    convert_temp();

    // TODO: Generate the nth Fibonacci number.

    // TODO: Print the lyrics to the Christmas carol “The Twelve Days of Christmas,”
    // taking advantage of the repetition in the song.
}

// Farenheit to Celcius, Celcius to Farenheit
fn convert_temp() {
    println!("Enter temperature in Farenheit:");
    let mut temperature_f = String::new();
    io::stdin()
        .read_line(&mut temperature_f)
        .expect("Failed reading line");
    let temperature_f: f32 = match temperature_f.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not valid temperature"),
    };

    let temp_in_f = convert_f(temperature_f);

    println!("This temperature in Celcius is {}", temp_in_f);

    println!("Enter temperature in Celcius:");
    let mut temperature_c = String::new();
    io::stdin()
        .read_line(&mut temperature_c)
        .expect("Failed reading line");
    let temperature_c: f32 = match temperature_c.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Not valid temperature"),
    };

    let temp_in_c = convert_c(temperature_c);

    println!("This temperature in Farenheit is {}", temp_in_c);
}

fn convert_f(temp: f32) -> f32 {
    (temp - 32.0) * (5.0 / 9.0)
}

fn convert_c(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) + (32.0)
}

