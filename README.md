# Rust: the completeguide

## Introduction

### Rust installation

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo --version
```

### Generate project

```sh
cargo new deck
cd deck
cargo run -q
```

## Core concepts

### Structs

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck = Deck { cards: vec![] };

    println!("My deck: {:?}", deck);
}
```

### Arrays vs Vectors

- Arrays: fixed size
- Vectors: dynamic size

```rust
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
```

### Implementations and methods

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // associated function
    fn new() -> Self {
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
        return deck;
    }

    // method
    fn shuffle(&self) {}
}

fn main() {
    let deck = Deck::new();
    deck.shuffle();

    // formatted output with #
    println!("My deck: {:#?}", deck);
}
```

### Implicit return

```rust
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // associated function
    fn new() -> Self {
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

        // implicit return (no ;)
        Deck { cards }
    }

    // method
    fn shuffle(&self) {}
}

fn main() {
    let deck = Deck::new();
    deck.shuffle();

    // formatted output with #
    println!("My deck: {:#?}", deck);
}
```

### External crates
