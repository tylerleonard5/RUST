use std::{io, vec};
use std::collections::HashMap;

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

    let mode_vec = mode(&mut input_vec);

    
    let length = (mode_vec.len() - 1) as i32;

    if length > 0 {
        print!("The modes are: ");
    }
    else{
        print!("The mode is: ");
    }

    let mut count = 0;
    for i in mode_vec{
        if count == length {
            print!("{}", i)
        } else{
            print!("{} and ", i);
            count += 1;
        }
    }

}

fn median(vec_input: &mut Vec<i32>) -> MedType {
    let length = vec_input.len() / 2;
    let length2 = vec_input.len();
    let length_int = length2 as i32;

    vec_input.sort();

    if length_int % 2 == 0 {
        return MedType::Float(((vec_input[length - 1]) + (vec_input[length])) as f32 / 2.0);
    } else {
        return MedType::Int(vec_input[length]);
    }
}

fn mode(vec_input: &mut Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();

    let mut modes: Vec<i32> = Vec::new();

    for i in vec_input {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    let mut count = 0; 

    for (key, value) in &map {
        if value > &count {
            let mut temp: Vec<i32> = Vec::new();
            temp.push(**key);
            modes = temp;
            count = *value;
        }else if value == &count {
            modes.push(**key)
        }
    }

    return modes;
} 