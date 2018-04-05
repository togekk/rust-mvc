extern crate stdweb;
extern crate regex;

use std::collections::HashMap;

mod mvc;

fn main() {
    let mut scope: HashMap<&str, &str> = HashMap::new();
    scope.insert("num", "22");
    scope.insert("name", "John");
    scope.insert("book", "Harry Potter");
    mvc::mvc(&scope);
}