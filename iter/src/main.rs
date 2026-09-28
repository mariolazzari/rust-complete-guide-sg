fn print_elememts(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_string(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter().map(|el| el.to_uppercase()).collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elememts(&colors);

    shorten_string(&mut colors[1..3]);
    println!("{:#?}", colors);

    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased);
}
