#[macro_use]
extern crate stdweb;

use std::collections::HashMap;

mod mvc;

fn main() {
    let html =
        "<div (class)=\"red\">Number:
            <span>{{num}}</span>
            <span>Name:</span>
            <span>{{name}}</span>
            <span>Book Name:</span>
            <span>{{book}}</span>
        </div>";
    let mut scope: HashMap<&str, &str> = HashMap::new();
    scope.insert("num", "22");
    scope.insert("name", "John");
    scope.insert("book", "Harry Potter");
    scope.insert("age", "30");
    mvc::mvc(html, &scope);
}