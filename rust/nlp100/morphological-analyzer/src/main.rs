extern crate mecab;
extern crate nlp100;

use mecab::Node;
use mecab::Tagger;
use nlp100::NLP100;
use std::collections::HashMap;

const ANALYZED_MECAB_KEYS: [&str; 9] = ["pos", "pos1", "pos2", "pos3", "a", "b", "base", "read", "speech"];

fn feature(node: &Node) -> HashMap<String, String> {
    let mut h: HashMap<String, String> = HashMap::new();
    let surface: String = (&(node.surface)[..node.length as usize]).to_string();
    h.insert("surface".to_string(), surface);
    let values: Vec<String> = node.feature.split(",").map(|m| m.to_string()).collect();
    for (a, b) in ANALYZED_MECAB_KEYS.iter().zip(values.iter()) {
        h.insert(a.to_string(), b.to_string());
    }
    h
}

fn verb(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    nodes.iter().filter(|m| m["pos"] == "動詞").map(|hm| hm.clone()).collect()
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
                    let m = feature(&node);
                    if m["surface"] == "の" && m["pos"] == "助詞" && m["pos1"] == "連体化" {
                        let prev = feature(&node.prev().unwrap());
                        let next = feature(&node.next().unwrap());

                        if prev["pos"] == "名詞" && next["pos"] == "名詞" {
                            println!("{}{}{}", &prev["surface"], &m["surface"], &next["surface"]);
                        }
                    }
                }
            }
        }
    }
}
