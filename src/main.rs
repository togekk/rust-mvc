#[macro_use]
extern crate stdweb;

use std::collections::HashMap;
use stdweb::web::{
    document,
    IElement
};

mod mvc;

fn main() {
    document().body().unwrap().set_attribute("style", "display: block").unwrap();
    let mut scope: HashMap<&str, &str> = HashMap::new();
    scope.insert("num", "22");
    scope.insert("name", "John");
    scope.insert("book", "Harry Potter");
    scope.insert("age", "30");
    mvc::mvc(&scope);
}