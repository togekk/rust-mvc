#[macro_use]
extern crate stdweb;

use std::collections::HashMap;

mod mvc;

fn main() {
    let mut scope: HashMap<&str, &str> = HashMap::new();
    scope.insert("num", "22");
    scope.insert("name", "John");
    scope.insert("book", "Harry Potter");
    scope.insert("age", "30");
    let html = "
    <div>
        <span>Number:</span>
        <div>{{num}}</div>
        <span>Name:</span>
        <span>{{name}}</span>
        <span>Book Name:</span>
        <span>{{book}}</span>
    </div>
    ";
    mvc::mvc(html, &scope);
}
