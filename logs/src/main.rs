use std::fs;

fn string_test(a: String, b: &String, c: &str) {}

fn main() {
    string_test(
        "red".to_string(),
        &String::from("red"),
        String::from("red").as_str(),
    );

    match fs::read_to_string("logs.txt") {
        Ok(was_read) => {
            println!("{:#?}", was_read.len());
        }
        Err(err) => {
            println!("error: {}", err)
        }
    }
}
