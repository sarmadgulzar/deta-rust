# data-rust

Deta SDK for Rust

Example:

```rust
use base::Base;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    key: String,
    name: String,
    age: i32,
}

fn main() {
    let base = Base::new(
        String::from("DB_NAME"),
        String::from("DB_PROJECT_KEY"),
        String::from("DB_PROJECT_ID"),
    );
    let user: User = base
        .get(String::from("krvb876h"))
        .expect("There was an error!")
        .json()
        .expect("There was an error!");

    println!("{:?}", user);
}
```

This will print the following to the console:

```raw
User { key: "krvb876h", name: "Sarmad", age: 25 }
```
