#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    // vars are unmutable by default
    let suits = ["Hearts", "Spades", "Diamonds"];
    let values = ["Ace", "Two", "Three"];

    // mutable var
    let mut cards = vec![];

    for suit in suits {
        for value in values {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    let deck = Deck { cards };

    // formatted output with #
    println!("My deck: {:#?}", deck);
}
