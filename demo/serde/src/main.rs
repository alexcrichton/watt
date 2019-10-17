#![allow(dead_code)]

#[derive(wa_serde_derive::Deserialize)]
struct Foo {
    a: f32,
    b: String,
    c: (String, i32),
    d: Option<u32>,
    e: u128,
    f: f64,
}

#[derive(serde_derive::Deserialize)]
struct Bar {
    a: f32,
    b: String,
    c: (String, i32),
    d: Option<u32>,
    e: u128,
    f: f64,
}

fn main() {
    println!("Hello, world!");
}
