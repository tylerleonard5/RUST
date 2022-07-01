use std::collections::HashMap;
use std::io;

fn main() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Retrieve or add employee to a project: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read input");

        let mut input_vec: Vec<String> = Vec::new();

        for word in input.split_whitespace() {
            input_vec.push(word.to_string());
        }

        if &input_vec[0] == "Retrieve" {
            let department = &input_vec[1];
            let department = String::from(department);

            let mut temp_vec: &Vec<String> = &Vec::new();

            let i = map.get(&department);

            match i {
                Some(a) => temp_vec = a,
                None => println!("Error"),
            }

            let mut temp_vec = temp_vec.clone();

            temp_vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

            for i in temp_vec {
                println!("Member: {}", i);
            }
        } else {
            let name = &input_vec[1];
            let name = String::from(name);

            let department = &input_vec[3];
            let department = String::from(department);

            let temp_vec: Vec<String> = Vec::new();

            let vec_map = map.entry(department).or_insert(temp_vec);
            vec_map.push(name);
        }
    }
}
