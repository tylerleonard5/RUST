fn main() {
    let mut count = 0;
    'counting_up: loop {
        // creates loop with label
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // label of loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // returns value from break
        }
    };

    println!("The result is {}", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
