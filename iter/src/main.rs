fn print_elememts(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn main() {
    let colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elememts(&colors);
}
