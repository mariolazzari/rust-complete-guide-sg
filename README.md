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

Reference looks to a value without moving it

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

fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let account = Account::new(1, String::from("Mario"));
    let account_ref = &account;

    print_account(account_ref);
    println!("{:#?}", account)
}
```

### Immutable reference

You can move any number of unmutable references, but you cannot mutate a readonly reference.

```rust
fn main() {
    let account = Account::new(1, String::from("Mario"));
    let account_ref1 = &account;
    let account_ref2 = &account;

    print_account(account_ref1);
    print_account(account_ref2);

    println!("{:#?}", account)
}
```

### Mutable reference

You can use mutable reference if no readable reference are present

```rust
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance = 10
}

fn main() {
    let mut account = Account::new(1, String::from("Mario"));

    change_account(&mut account);
    print_account(&account);
}
```

### Copy-able values

Primitive values art copied, not moved.

```rust
fn main() {
    let num = 5;

    let other_num = num;

    print!("{} {}", num, other_num)
}
```

## Lifetimes

### Basics

- Lifetime: how log an owner or reference exists
- Generic lifetimes: extra syntex added to clarify relationships between lifetimes
- Out of scope, owner and reference are dropped

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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("Mario"));

    bank.add_account(account);
    println!("{:#?}", bank);
}
```

### Bank project

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

    fn summary(&self) -> String {
        format!("{} has a balance of {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
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

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

fn main() {
    let mut bank = Bank::new();

    let mut account = Account::new(1, String::from("Mario"));
    account.deposit(500);
    account.withdraw(250);
    print!("{}", account.summary());

    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}
```

## Pattern matching and option

### Catalog project

```sh
cargo new media
```

### Enums

```rust
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::AudioBook {
        title: String::from("My AudioBook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
```

### Adding implementations to Enums

```rust
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        if let Media::Book { title, author } = self {
            format!("Book: {} {}", title, author)
        } else if let Media::Movie { title, director } = self {
            format!("Movie: {} {}", title, director)
        } else if let Media::Audiobook { title } = self {
            format!("Audiobook: {}", title)
        } else {
            String::from("Media description")
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My AudioBook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    audiobook.description();

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
```

### Pattern matching with Enums

```rust
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media)
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My AudioBook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    audiobook.description();

    print_media(audiobook);
    print_media(good_movie);
    print_media(bad_book);
}
```

### Structs vs Enums

- Same methods -> enumas
- Different methods -> structs
- Lots of properties -> match too dense -> structs

### Catalog item

```rust
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media.description())
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My AudioBook"),
    };

    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };

    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_book);

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);

    println!("{:#?}", catalog);
}
```

### Unlabeled fields

```rust
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media.description())
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My AudioBook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_book);

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    println!("{:#?}", catalog);
}
```

### Option Enum

- no null, nil or undefined
- built-in enum Option
- Some or None
- pattern matching
- must handle two cases: value or not

```rust
#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have somethign to return
            Some(&self.items[index])
        } else {
            // Bad! We don't have anything to return!!!
            None
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media.description())
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My AudioBook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_book);

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("{:#?}", value);
        }
        Option::None => {
            println!("no value...");
        }
    }

    match catalog.get_by_index(9999) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }
    }
}
```

### Other ways to handle options

[Docs](https://doc.rust-lang.org/std/option/)

- unwrap: quick debug
- expect: crash if no value
- unwrap_or: fallback

```rust
use crate::Media::Placeholder;

#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book: {} {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie: {} {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if self.items.len() > index {
            // Good! We have somethign to return
            Some(&self.items[index])
        } else {
            // Bad! We don't have anything to return!!!
            None
        }
    }
}

fn print_media(media: Media) {
    println!("{:#?}", media.description())
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("My AudioBook"),
    };
    let good_movie = Media::Movie {
        title: String::from("Good Movie"),
        director: String::from("Good Director"),
    };
    let bad_book = Media::Book {
        title: String::from("Bad Book"),
        author: String::from("Bad Author"),
    };
    let podcast = Media::Podcast(10);
    let placeholder = Media::Placeholder;

    // print_media(audiobook);
    // print_media(good_movie);
    // print_media(bad_book);

    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(good_movie);
    catalog.add(bad_book);
    catalog.add(podcast);
    catalog.add(placeholder);

    match catalog.items.get(0) {
        Option::Some(value) => {
            println!("{:#?}", value);
        }
        Option::None => {
            println!("no value...");
        }
    }

    match catalog.get_by_index(9999) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("No value here!");
        }
    }

    let item = catalog.get_by_index(9999);
    let placeholder = Media::Placeholder;
    // println!("Item: {:#?}", item.unwrap()); // panic on 9999
    // println!("Item: {:#?}", item.expect("expteced value")); // panic on 9999
    println!("Item: {:#?}", item.unwrap_or(&placeholder));
}
```

## Modules

### Modules overview

Group togheter related piece of codes.

### Rules on modules

- mod
- use
- pub

### Multiple modules

```rust
pub mod catalog;
pub mod media;
```

## Errors and results

### Project overview

```sh
cargo new logs
```

### Read file

```rust
use std::fs;

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);
}
```

### Result enum

Result<T,E>

```rust
fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
```

### Matching results

```rust
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
```

### Empty Ok vatiant

```rust
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

    match validate_email(String::from("mario.lazzari@gmail.com")) {
        Ok(..) => println!("email is valid"),
        Err(reason) => {
            println!("{}", reason)
        }
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("invalid email address"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
```

### Reading files

```rust
use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(was_read) => {
            println!("{:#?}", was_read.len());
        }
        Err(err) => {
            println!("error: {}", err)
        }
    }
}
```

### Strings

```rust
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
```

### Stack and Heap

- Stack: fast, limited, fixed size (2-8 MB)
  - Metadata
- Heap: slow, dynamic, allocates memory
  - Data

### Strings

- String: a growable, UTF-8 encoded string
  - Ownership of a text
  - Can grow or shrink
- String ref: a reference to a String
  - Rarely used
  - Portion of a String
- String slice: a reference to a part of a String
  - Read a string owned by another variable

### Finding error logs

```rust
use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split('\n');
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(was_read) => {
            error_logs = extract_errors(was_read.as_str());
        }
        Err(err) => {
            println!("error: {}", err)
        }
    }

    println!("{:#?}", error_logs);
}
```

### Writing data

```rust
use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split('\n');
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(was_read) => {
            let error_logs = extract_errors(was_read.as_str());
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Wrote error.txt"),
                Err(reason) => {
                    println!("Writing errors.txt failed: {}", reason)
                }
            }
        }
        Err(err) => {
            println!("error: {}", err)
        }
    }
}
```

### Nested matches alternatives

```rust
fn main() {
    let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt");
}
```
