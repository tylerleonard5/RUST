use reqwest::StatusCode;

mod utils;

use std::fs;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let url = "https://spactrack.io/spacs";
    let result = client.get(url).send().await.unwrap();

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };

    println!("HTML: {}", raw_html);

    fs::write("./foo", raw_html).expect("Unable to write file");
}