extern crate stdweb;

use std::collections::HashMap;
use std::vec::Vec;
use stdweb::web::{document, INode, IParentNode, Node, NodeList};

pub fn init(html: &str, scope: &HashMap<&str, String>) {
    let app = document().query_selector("app-component").unwrap().unwrap();
    if app.has_child_nodes() {
        app.remove_child(&app.first_child().unwrap()).unwrap();
    }
    let frag = document().create_document_fragment();
    let div = document().create_element("div").unwrap();
    js!(@{&div}.innerHTML = @{&html});

    let node_list = div.child_nodes();
    let mut mvc_node_list: Vec<Node> = Vec::new();
    let count: i64 = 0;
    if render_models(node_list, &mut mvc_node_list, scope, count) {
        frag.append_child(&div);
        app.append_child(&frag);
    };
}

fn render_models(
    node_list: NodeList,
    mvc_node_list: &mut Vec<Node>,
    scope: &HashMap<&str, String>,
    mut count: i64,
) -> bool {
    let len = node_list.len();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        let mut text = node.text_content().unwrap();

        let child_node_list = node.child_nodes();
        if child_node_list.len() > 0 {
            count += 1;
            render_models(child_node_list, mvc_node_list, scope, count);
            count -= 1;
        } else {
            for (key, val) in scope {
                let mut key_new: String = "{{".to_owned();
                key_new.push_str(key);
                key_new.push_str("}}");
                text = text.replace(&key_new, val);
                mvc_node_list.push(node.clone());
            }

            js!(@{&node}.textContent = @{text});
        }
    }

    return true;
}
