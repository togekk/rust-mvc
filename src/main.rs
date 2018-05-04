#[macro_use]
extern crate stdweb;

use std::collections::HashMap;
use stdweb::traits::IMouseEvent;
use stdweb::web::event::ClickEvent;
use stdweb::web::event::MouseMoveEvent;
use stdweb::web::{window, IEventTarget};

mod mvc;

fn main() {
    static HTML: &'static str = include_str!("../static/demo.html");
    let mut scope: HashMap<&str, String> = HashMap::new();

    scope.insert("name", "John".to_owned());
    scope.insert("x", "null".to_owned());
    scope.insert("y", "null".to_owned());
    mvc::render(HTML, &scope);
    let mut scope_2 = scope.clone();

    window().add_event_listener(move |event: MouseMoveEvent| {
        let x: String = (f64::from(event.client_x()) * 3.141592654).to_string();
        let y: String = (f64::from(event.client_y()) * 3.141592654).to_string();
        *scope.get_mut("x").unwrap() = x;
        *scope.get_mut("y").unwrap() = y;
        mvc::render(HTML, &scope);
    });

    window().add_event_listener(move |_: ClickEvent| {
        *scope_2.get_mut("name").unwrap() = "David".to_owned();
        *scope_2.get_mut("x").unwrap() = "click!".to_owned();
        *scope_2.get_mut("y").unwrap() = "click!".to_owned();
        mvc::render(HTML, &scope_2);
    });
}
