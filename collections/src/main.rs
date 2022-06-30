fn main() {
   enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    
    let x = &row[0];

    let mut y = 0;

    match x {
        SpreadsheetCell::Int(i) => y = i + 6,
        _ => ()
    }

    println!("{}", y);
}
