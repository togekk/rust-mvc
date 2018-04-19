extern crate stdweb;

use stdweb::web::{
    document,
    INode,
    INonElementParentNode
};

fn main() {
    let frag = document().create_document_fragment();

    for _i in 0..10 {
        let div = document().create_element("div").unwrap();
        let text = document().create_text_node("test");
        div.append_child(&text);
        frag.append_child(&div);
    }

    let el = document().get_element_by_id("to_be_append").unwrap();
    el.append_child(&frag);
}