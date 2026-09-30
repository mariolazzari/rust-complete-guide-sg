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

fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}

fn find_color_or(elements: &[String], search: &str, callback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(callback), |el| el.to_string())
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    print_elememts(&colors);

    shorten_string(&mut colors[1..3]);
    println!("Colors: {:#?}", colors);

    let uppercased = to_uppercase(&colors);
    println!("Upperscaled: {:#?}", uppercased);

    let exploded = explode(&colors);
    println!("Eploded: {:#?}", exploded);

    let found_color = find_color_or(&colors, "re", "orange");
    println!("Found: {:#?}", found_color);

    let mut destination = vec![];
    move_elements(colors, &mut destination);
    println!("Destination: {:#?}", destination);
}
