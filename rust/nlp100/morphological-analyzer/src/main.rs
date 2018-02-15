extern crate mecab;
extern crate nlp100;

use mecab::Node;
use mecab::Tagger;
use nlp100::NLP100;
use std::collections::HashMap;

const ANALYZED_MECAB_KEYS: [&str; 9] = ["pos", "pos1", "pos2", "pos3", "a", "b", "base", "read", "speech"];
const VERB: &str = "動詞";
const NOUN: &str = "名詞";
const PARTICLE: &str = "助詞";

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

fn between_noun(node: &Node) -> Option<String> {
    let mecab = feature(node);
    if mecab["surface"] == "の" && mecab["pos"] == PARTICLE && mecab["pos1"] == "連体化" {
        let prev = feature(&node.prev().unwrap());
        let next = feature(&node.next().unwrap());

        Some(format!("{}{}{}", &prev["surface"], &mecab["surface"], &next["surface"]))
    } else {
        None
    }
}

fn verb(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    nodes.iter().filter(|m| m["pos"] == VERB).map(|hm| hm.clone()).collect()
}

fn noun(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    nodes.iter().filter(|node| node["pos"] == NOUN).map(|hm| hm.clone()).collect()
}

fn sa_noun(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>>{
    noun(nodes).iter().filter(|node| node["pos1"] == "サ変接続").map(|hm| hm.clone()).collect()
}

fn main() {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt".to_string();
    let neco: Vec<String> = NLP100::get(url).split("\n").filter(|f| f.ne(&"")).map(|m| m.to_string()).collect();

    for line in neco {
        let mut tagger: Tagger = mecab::Tagger::new("");
        let mut mecabu: String = String::from("");
        let nodes: Node = tagger.parse_to_node(line);

        for node in nodes.iter_next() {
            match node.stat as i32 {
                mecab::MECAB_BOS_NODE => (),
                mecab::MECAB_EOS_NODE => (),
                _ => {
                    let m = feature(&node);
                    match m["pos"].as_ref() {
                        NOUN => {
                            mecabu = mecabu + &m["surface"];
                        },
                        _ => {
                            match mecabu.as_ref() {
                                "" => (),
                                _ => {
                                    println!("{}", mecabu);
                                    mecabu = String::from("");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
