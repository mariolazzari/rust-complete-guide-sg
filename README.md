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
