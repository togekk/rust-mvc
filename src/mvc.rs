extern crate stdweb;

use std::collections::HashMap;
use stdweb::unstable::TryInto;
use stdweb::web::{document, INode, IParentNode, Node, NodeList};

pub fn render(html: &str, scope: &HashMap<&str, String>) {
    let app = document().query_selector("app-component").unwrap().unwrap();
    let frag = document().create_document_fragment();
    let div = document().create_element("div").unwrap();
    js!(@{&div}.innerHTML = @{&html});

    let node_list = div.child_nodes();
    let mut nodes_to_delete: Vec<Node> = Vec::new();
    let count: i64 = 0;
    if render_models(node_list, scope, count, &mut nodes_to_delete) {
        remove_nodes(&mut nodes_to_delete);
        frag.append_child(&div);
        if app.has_child_nodes() {
            app.replace_child(&frag, &app.first_child().unwrap())
                .unwrap();
        } else {
            app.append_child(&frag);
        }
    };
}

fn render_models(
    node_list: NodeList,
    scope: &HashMap<&str, String>,
    mut count: i64,
    nodes_to_delete: &mut Vec<Node>,
) -> bool {
    let len = node_list.len();

    for i in 0..len {
        let node = node_list.item(i).unwrap();
        let mut show = true;
        let attr_len = get_length(&node);
        for i in 0..attr_len {
            let a = get_item(&node, i);
            match a {
                Some(attr) => {
                    if get_key(&attr) == "(show)" && !eval(&attr, scope) {
                        show = false;
                    };
                }
                None => {}
            }
        }

        if !show {
            nodes_to_delete.push(node.clone());
        } else {
            let mut text = node.text_content().unwrap();
            let child_node_list = node.child_nodes();

            if child_node_list.len() > 0 {
                count += 1;
                render_models(child_node_list, scope, count, nodes_to_delete);
                count -= 1;
            } else {
                // Models
                for (key, val) in scope {
                    let mut key_new: String = "{{".to_owned();
                    key_new.push_str(key);
                    key_new.push_str("}}");
                    text = text.replace(&key_new, val);
                }

                js!(@{&node}.textContent = @{text});
            }
        }
    }

    return true;
}

fn remove_nodes(nodes_to_delete: &mut Vec<Node>) {
    for i in 0..nodes_to_delete.len() {
        nodes_to_delete[i]
            .parent_node()
            .unwrap()
            .remove_child(&nodes_to_delete[i])
            .unwrap();
    }
}

pub fn set_scope(scope: &mut HashMap<&str, String>, key: &str, val: &str) {
    *scope.get_mut(key).unwrap() = val.to_owned();
}

fn get_length(node: &Node) -> u32 {
    js!(
        var a = 0;
        if (!!@{&node}.attributes)
        a = @{&node}.attributes.length;
        return a;
    ).try_into()
        .unwrap()
}

fn get_item(node: &Node, index: u32) -> Option<Node> {
    js!(
        var a;
        if (!!@{&node}.attributes)
        a = @{node}.attributes[@{index}];
            return a;
        ).try_into()
        .unwrap()
}

fn get_value(attr: &Node) -> String {
    js!(return @{attr}.value).try_into().unwrap()
}

fn get_key(attr: &Node) -> String {
    js!(return @{attr}.name).try_into().unwrap()
}

fn eval(attr: &Node, scope: &HashMap<&str, String>) -> bool {
    let attr_value: &str = &get_value(attr);
    let scope_value = scope.get(attr_value).unwrap();
    return eval_string(scope_value.to_string());
}

fn eval_string(str: String) -> bool {
    js!(return eval(@{str})).try_into().unwrap()
}
