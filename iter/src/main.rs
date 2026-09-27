fn print_elememts(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_string(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    print_elememts(&colors);

    shorten_string(&mut colors[1..3]);
    println!("{:#?}", colors)
}
