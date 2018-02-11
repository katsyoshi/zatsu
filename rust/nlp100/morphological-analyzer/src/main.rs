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

fn verb(nodes: Vec<HashMap<String, String>>) {
    for verb in nodes.iter().filter(|m| m["pos"] == "動詞") {
        println!("{}: {}", verb["surface"], verb["base"]);
    }
}

fn noun(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    nodes.iter().filter(|node| node["pos"] == "名詞").map(|hm| hm.clone()).collect()
}

fn sa_noun(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>>{
    noun(nodes).iter().filter(|node| node["pos1"] == "サ変接続").map(|hm| hm.clone()).collect()
}

fn main() {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt".to_string();
    let neco: Vec<String> = NLP100::get(url).split("\n").filter(|f| f.ne(&"")).map(|m| m.to_string()).collect();

    for line in neco {
        let mut tagger: Tagger = mecab::Tagger::new("");
        let mut mecab: Vec<HashMap<String, String>> = Vec::new();
        let nodes: Node = tagger.parse_to_node(line);

        for node in nodes.iter_next() {
            match node.stat as i32 {
                mecab::MECAB_BOS_NODE => (),
                mecab::MECAB_EOS_NODE => (),
                _ => {
                    mecab.push(feature(node));
                }
            }
        }

        for noun in sa_noun(mecab) {
            println!("{}: {}", noun["surface"], noun["pos"]);
        }
    }
}
