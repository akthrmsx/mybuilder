Derive macro: `derive(Builder)`

https://github.com/dtolnay/proc-macro-workshop

```rs
use mybuilder::Builder;

#[derive(Debug, Clone, Builder)]
struct Udon {
    size: Size,
    toppings: Vec<Topping>,
    note: Option<String>,
}

#[derive(Debug, Clone)]
enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone)]
enum Topping {
    Meat,
    Tempura,
    Wakame,
}

fn main() {
    let mut builder = Udon::builder();
    let udon = builder
        .size(Size::Medium)
        .toppings(vec![Topping::Meat, Topping::Tempura])
        .note("without green onions".into())
        .build()
        .unwrap();

    println!("{:?}", udon);
}
```
