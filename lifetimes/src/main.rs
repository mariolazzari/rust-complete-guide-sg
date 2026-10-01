fn next_languages<'a>(languages: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}

// elision of annotation
fn last_language(languages: &[String]) -> &str {
    languages.last().unwrap()
}

fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str {
    if lang_a.len() >= lang_b.len() {
        lang_a
    } else {
        lang_b
    }
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let next_language = next_languages(&languages, "go");
    println!("{}", next_language);

    let last_lang = last_language(&languages);
    println!("{}", last_lang);

    let long_lang = longest_language("go", "typescript")
    println!("{}", long_lang);
}
