extern crate stdweb;

use std::collections::HashMap;
use stdweb::web::{
    document,
    INode
};

pub fn mvc(scope: &HashMap<&str, &str>) {

    let body = document().body().unwrap();
    let node_list = body.child_nodes();
    let len = node_list.len();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        // let parent_node = node.parent_node().unwrap();
        let text = node.text_content().unwrap();

        if let Some(_start) = text.find("{{") {
            // let node_new = document().create_element(&node.node_name()).unwrap();
            // while (node.first_child().is_null()) {
                node.remove_child(&node.first_child().unwrap()).unwrap();
            // }

            let split = text.split("{{");

            for s in split {

                if let Some(end) = s.find("}}") {
                    let t = &s[0..end];

                    match scope.get(t) {
                        Some(scope_found) => {
                            let item = document().create_text_node(&scope_found);
                            node.append_child(&item);
                        },
                        None => println!("")
                    }

                    let rest = &s[end+2..];
                    let item = document().create_text_node(&rest);
                    node.append_child(&item);

                } else {

                    let item = document().create_text_node(&s);
                    node.append_child(&item);
                    
                }

            }

            // parent_node.remove_child(&node).unwrap();

            // node.append_child(&node_new);
        }

        

    }

}