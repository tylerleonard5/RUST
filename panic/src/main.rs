use std::fs;

fn main() {
    let f = fs::read_to_string("hello.txt").unwrap();

    println!("{}", f);
}
