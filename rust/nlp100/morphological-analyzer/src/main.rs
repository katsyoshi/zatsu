extern crate mecab;
extern crate nlp100;

use mecab::Node;
use mecab::Tagger;
use nlp100::NLP100;
use std::collections::HashMap;

fn feature(node: Node) -> HashMap<String, String> {
    let mut h: HashMap<String, String> = HashMap::new();
    let surface: String = (&(node.surface)[..node.length as usize]).to_string();
    h.insert("surface".to_string(), surface);
    let keys: Vec<String> = vec!["pos", "pos1", "pos2", "pos3", "a", "b", "base", "read", "speech"].iter().map(|m| m.to_string()).collect();
    let values: Vec<String> = node.feature.split(",").map(|m| m.to_string()).collect();
    for (a, b) in keys.iter().zip(values.iter()) {
        h.insert(a.to_string(), b.to_string());
    }
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
    mecab.iter().filter(|m| m["pos"] == "動詞");
}
