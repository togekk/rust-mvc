extern crate stdweb;

use std::collections::HashMap;
use stdweb::web::{document, INode, IParentNode, NodeList};

pub fn init(html: &str, scope: &HashMap<&str, &str>) {
    let app = document().query_selector("app-component").unwrap().unwrap();
    // app.remove_child(&app.first_child().unwrap()).unwrap();
    let frag = document().create_document_fragment();
    let div = document().create_element("div").unwrap();
    js!(@{&div}.innerHTML = @{&html});

    let node_list = div.child_nodes();
    let count: i64 = 0;
    if render_models(node_list, scope, count) {
        frag.append_child(&div);
        app.append_child(&frag);
    };
}

fn render_models(node_list: NodeList, scope: &HashMap<&str, &str>, mut count: i64) -> bool {
    let len = node_list.len();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        let mut text = node.text_content().unwrap();

        let child_node_list = node.child_nodes();
        if child_node_list.len() > 0 {
            count += 1;
            render_models(child_node_list, scope, count);
            count -= 1;
        } else {
            for (key, val) in scope {
                let mut key_new: String = "{{".to_owned();
                key_new.push_str(key);
                key_new.push_str("}}");
                text = text.replace(&key_new, val);
            }

            js!(@{&node}.textContent = @{text});
        }
    }

    return true;
}

// pub fn mousemove() {
//     console!(log, "OK");
// }
