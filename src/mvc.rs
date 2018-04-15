extern crate stdweb;

use std::collections::HashMap;
use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    HtmlElement,
    INode,
    Node,
    NodeList,
    DocumentFragment
};
use std::vec::Vec;

pub fn mvc(_html: &str, scope: &HashMap<&str, &str>) {

    let body = document().body().unwrap();
    let node_list = body.child_nodes();
    let frag = document().create_document_fragment().unwrap();
    // let aaa = node_list.item(1).unwrap().get_attribute("(class)").unwrap();
    // let item = document().create_text_node(&aaa);
    let mut scope_found_id: Vec<u32> = Vec::new();
    render_models(node_list, scope, &frag, &mut scope_found_id);
    if remove_nodes(&body, body.child_nodes(), scope_found_id) {
        body.append_child(&frag);
    };
}

fn render_models(node_list: NodeList, scope: &HashMap<&str, &str>, frag: &DocumentFragment, mut scope_found_id: &mut Vec<u32>) -> bool {
    
    let len = node_list.len();
    let mut found = false;
    let mut a = false;

    for i in 0..len {
        let node = node_list.item(i).unwrap();

                if get_type(node_list.item(i).unwrap()) == 8 {
            let node_comment = node_list.item(i).unwrap();
            js!(@{node_comment}.textContent = "");
        }

        let text = node.text_content().unwrap();

        if let Some(_start) = text.find("{{") {
            found = true;
            scope_found_id.push(i);

            match get_type(node_list.item(i).unwrap()) {
                1 => {
                    node.remove_child(&node.first_child().unwrap()).unwrap();
                },
                _ => (),
            }

            let split = text.split("{{");

            for s in split {

                if let Some(end) = s.find("}}") {
                    let t = &s[0..end];

                    match scope.get(t) {
                        Some(scope_found) => {
                            let item = document().create_text_node(&scope_found);
                            frag.append_child(&item);
                        },
                        None => println!("")
                    }

                    let rest = &s[end+2..];
                    if get_type(node_list.item(i).unwrap()) != 8 {
                        let item = document().create_text_node(&rest);
                        frag.append_child(&item);
                    }

                } else {
                    let item = document().create_text_node(&s);
                    frag.append_child(&item);
                }

            }

        } else {
            if !a {
                let item = document().create_text_node(&text);
                frag.append_child(&item);
            }
        }

        let child_node_list = node.child_nodes();
        if child_node_list.len() > 1 {
            a = render_models(child_node_list, scope, frag, &mut scope_found_id);
        };



    }

    return found;
}

fn remove_nodes(body: &HtmlElement, node_list: NodeList, scope_found_id: Vec<u32>) -> bool {
    let len = node_list.len();

    let mut a: Vec<u32> = Vec::new();

    for i in 0..len {
        if scope_found_id.contains(&i) {
            if get_type(node_list.item(i).unwrap()) != 3  {
                a.push(i);
            } else {
                let node = node_list.item(i).unwrap();
                js!(@{node}.textContent = "");
            }
        }
    }

    for i in 0..a.len() {
        body.remove_child(&node_list.item(a[i]).unwrap()).unwrap();
    }
    
    return true;
}

fn get_type(node: Node) -> u32 {
    let a = js!( return @{node}.nodeType );
    let b: u32 = a.try_into().unwrap();
    return b;
}