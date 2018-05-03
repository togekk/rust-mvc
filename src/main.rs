#[macro_use]
extern crate stdweb;

use std::collections::HashMap;
use stdweb::traits::IMouseEvent;
use stdweb::web::event::MouseMoveEvent;
use stdweb::web::{window, IEventTarget};

mod mvc;

fn main() {
    // let a: String = f64::from(3.141592654 / 2.5816 * 323542.0).to_string();
    static HTML: &'static str = include_str!("../static/demo.html");
    let mut scope: HashMap<&str, &str> = HashMap::new();

    scope.insert("name", "John");
    scope.insert("x", "999");
    mvc::init(HTML, &scope);

    // window().add_event_listener(move |event: MouseMoveEvent| {
    //     let x = f64::from(event.client_x());
    //     console!(log, x);
    //     scope = HashMap::new();
    //     scope.insert("name", "John");

    //     scope.insert("x", &x.to_string());
    // mvc::init(HTML, &scope);
    // });
}
