extern crate stdweb;
extern crate regex;

use regex::Regex;
use stdweb::web::{
    document,
    INode
    // INonElementParentNode
};

use std::collections::HashMap;


fn main() {
    // let add = | name: &str | {
    //     let mut a:String = "hello ".to_owned();
    //     a.push_str(name);
    //     return a;
    // };
    let re = Regex::new(r"\{\{.*\}\}").unwrap();

    let body = document().body().unwrap();
    let node_list = body.child_nodes();
    let len = node_list.len();

    let ng_model: HashMap<&mut str, stdweb::web::Node> = HashMap::new();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        let text = node.text_content().unwrap();
        if re.is_match(&*text) {
            let bbb = node.first_child().unwrap();
            node.remove_child(&bbb).unwrap();
            ng_model["aaa"] = node;
        }
    }

    let aaa = document().create_text_node("ssss");

    ng_model["aaa"].append_child(&aaa);


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