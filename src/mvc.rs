extern crate stdweb;

use std::collections::HashMap;
use stdweb::web::{
    document,
    INode,
    NodeList
};

pub fn mvc(scope: &HashMap<&str, &str>) {

    let body = document().body().unwrap();
    let node_list = body.child_nodes();
    let count: i64 = 0;
    render_models(node_list, scope, count);
}

fn render_models(node_list: NodeList, scope: &HashMap<&str, &str>, mut count: i64) -> bool {
    
    let len = node_list.len();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        let mut text = node.text_content().unwrap();

        if count > 0 {
            for (key, val) in scope {
                let mut key_new: String = "{{".to_owned();
                key_new.push_str(key);
                key_new.push_str("}}");
                text = text.replace(&key_new, val);
            }

            js!(@{&node}.textContent = @{text});
        }

        let child_node_list = node.child_nodes();
        if child_node_list.len() > 0 {
            count += 1;
            render_models(child_node_list, scope, count);
            count -= 1;
        };
    }

    return true;
}