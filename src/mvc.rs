extern crate stdweb;
extern crate regex;

use std::collections::HashMap;
use stdweb::web::{
    document,
    INode
};
use regex::Regex;

pub fn mvc(scope: &HashMap<&str, &str>) {
    let re = Regex::new(r"\{\{.*\}\}").unwrap();

    let body = document().body().unwrap();
    let node_list = body.child_nodes();
    let len = node_list.len();

    let mut ng_model: HashMap<String, stdweb::web::Node> = HashMap::new();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        let text = node.text_content().unwrap();
        // let aaa = document().create_text_node(&text);
        // body.append_child(&aaa);
        if re.is_match(&*text) {
            let bbb = node.first_child().unwrap();
            node.remove_child(&bbb).unwrap();
            ng_model.insert(text, node);
        }
    }

    for (key, value) in &ng_model {
        let last = key.len() - 2;
        let key_new = &key[2..last];
        match scope.get(key_new) {
            Some(scope_found) => {
                let aaa = document().create_text_node(&scope_found);
                value.append_child(&aaa);
            },
            None => println!("")
        }
    }


    // let div_root = document().create_element("div").unwrap();

    // for i in 0..10 {
    //     let div = document().create_element("div").unwrap();
    //     let num: &str = &*i.to_string();
    //     let text = document().create_text_node(num);
    //     div.append_child(&text);
    //     div_root.append_child(&div);
    // }

    // aa.append_child(&div_root);
}