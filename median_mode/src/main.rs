use std::io;

enum MedType {
    Float(f32),
    Int(i32),
}

fn main() {
    println!("Enter list of integers, separated by spaces");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read input");

    let mut input_vec: Vec<i32> = Vec::new();

    for word in input.split_whitespace() {
        input_vec.push(word.parse::<i32>().unwrap());
    }
    let x = median(&mut input_vec);
    match x {
        MedType::Float(value) => println!("The median is: {}", value),
        MedType::Int(value) => println!("The median is: {}", value),
    }
}

fn median(vec_input: &mut Vec<i32>) -> MedType {
    let length = vec_input.len() / 2;
    let length2 = vec_input.len();
    let length_int = length2 as i32;

    vec_input.sort();

    if length_int % 2 == 0 {
        return MedType::Float(((vec_input[length - 1] as f32) + (vec_input[length] as f32)) / 2.0);
    } else {
        return MedType::Int(vec_input[length]);
    }
}
