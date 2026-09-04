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

- [Rust stdlib](https://doc.rust-lang.org/std/)
- [Creates](https://crates.io/)
- [Docs](https://docs.rs/)

```sh
cargo add rand
```

```toml
[package]
name = "deck"
version = "0.1.0"
edition = "2024"

[dependencies]
rand = "0.10.2"
```

### Using crates

```rust
use rand::{rng, seq::SliceRandom};

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
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    // formatted output with #
    println!("My deck: {:#?}", deck);
}
```

### Splitting vectors

[Split](https://doc.rust-lang.org/stable/std/?search=split)

```rust
use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    // associated functions
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

    // methods
    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let cards = deck.deal(3);

    println!("My cards: {:#?}", cards);
    println!("My deck: {:#?}", deck);
}
```

## Ownership and borrowing

### Structs

```rust
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank{
    fn new()-> Self{
        
    }
}

fn main() {
    println!("Hello, world!");
}
```

### Inherent implementatios

```rust
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn main() {
    
    println!("Hello, world!");
}
```

### Use of moved value

```rust
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("Mario"));

    println!("{:#?}", bank);
    print_account(account);
    // print_account(account); -> use of moved value
}
```

### Ownership basics

- Every value is owned by a single variable at a time
- Reasigning the value to another variable moves the value: original variable cannot be used

```rust
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn main() {
    let mut account = Account::new(1, String::from("Mario"));

    account = print_account(account);
    account = print_account(account);

    println!("{:#?}", account)
}
```

### Borrow system
