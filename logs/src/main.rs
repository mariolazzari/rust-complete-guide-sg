use std::fs;
use std::io::Error;

fn main() {
    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);

    match divide(5.0, 3.0) {
        Ok(result) => {
            println!("result of division: {:#?}", result);
        }
        Err(err) => {
            println!("error: {}", err);
        }
    };
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
