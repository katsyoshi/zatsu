extern crate mecab;
extern crate nlp100;

use mecab::Node;
use mecab::Tagger;
use nlp100::NLP100;
use std::collections::HashMap;

fn feature(node: Node) -> HashMap<String, String> {
    let mut h: HashMap<String, String> = HashMap::new();
    let surface: String = (&(node.surface)[..node.length as usize]).to_string();
    let feature: Vec<String> = node.feature.split(",").map(|m| m.to_string()).collect();
    h.insert("surface".to_string(), surface);
    h.insert("base".to_string(), feature[0].to_string());
    h.insert("pos".to_string(), feature[1].to_string());
    h.insert("pos1".to_string(), feature[2].to_string());
    h
}

fn main() {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt".to_string();
    let neco = NLP100::get(url);
    let mut tagger: Tagger = mecab::Tagger::new("");
    let nodes: Node = tagger.parse_to_node(neco);
    let mut mecab: Vec<HashMap<String, String>> = Vec::new();

    for node in nodes.iter_next() {
        match node.stat as i32 {
            mecab::MECAB_BOS_NODE => (),
            mecab::MECAB_EOS_NODE => (),
            _ => {
                mecab.push(feature(node));
            }
        }
    }
}
