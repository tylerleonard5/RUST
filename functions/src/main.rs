fn main() {
    println!("{}", print_labeled_measurement(5, 'h'));
}

fn print_labeled_measurement(value: i32, unit_label: char) -> i32 {
    println!("The measurement is: {}{}", value, unit_label);

    return 3; // Or like this if done early

    4 // can return like this in RUST
}


