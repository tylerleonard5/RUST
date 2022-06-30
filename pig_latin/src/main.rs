use std::io;

fn main() {
    println!("Enter string to convert to pig latin: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read input");

    let temp = String::from(&input);
    let slice = &temp[1..];

    println!("Word in pig latin: {}", pig_latin(input, slice));
}

fn pig_latin(input: String, slice: &str) -> String {
    let mut x = input.chars();
    let mut temp = String::from(input.clone().trim());
    let first = x.nth(0).unwrap();
    let str_hay = "-hay";
    match first {
        'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => temp.push_str(str_hay),
        _ => temp = format!("{}-{}ay", slice.trim(), first),
    }

    return temp;
}
