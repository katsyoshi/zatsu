extern crate gnuplot;
extern crate mecab;
extern crate nlp100;

use gnuplot::*;
use mecab::Node;
use mecab::Tagger;
use nlp100::NLP100;
use std::collections::HashMap;

const ANALYZED_MECAB_KEYS: [&str; 9] = ["pos", "pos1", "pos2", "pos3", "a", "b", "base", "read", "speech"];
enum PartOfSpeech {
    VERB,
    NOUN,
    PARTICLE,
}

use PartOfSpeech::*;

fn inspect(val: PartOfSpeech) -> String {
    match val {
        VERB => "動詞",
        NOUN => "名詞",
        PARTICLE => "助詞",
    }.to_string()
}

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

fn word_histgram(nodes: Vec<HashMap<String, String>>) -> HashMap<String, u64> {
    let mut results: HashMap<String, u64> = HashMap::new();
    for node in nodes {
        let base = &node["base"];
        *results.entry(base.to_string()).or_insert(0) += 1;
    }
    results
}

fn between_noun(node: &Node) -> Option<String> {
    let mecab = feature(node);
    if mecab["surface"] == "の" && mecab["pos"] == inspect(PARTICLE) && mecab["pos1"] == "連体化" {
        let prev = feature(&node.prev().unwrap());
        let next = feature(&node.next().unwrap());

        Some(format!("{}{}{}", &prev["surface"], &mecab["surface"], &next["surface"]))
    } else {
        None
    }
}

fn verb(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    nodes.iter().filter(|m| m["pos"] == inspect(VERB)).map(|hm| hm.clone()).collect()
}

fn noun(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>> {
    nodes.iter().filter(|node| node["pos"] == inspect(NOUN)).map(|hm| hm.clone()).collect()
}

fn sa_noun(nodes: Vec<HashMap<String, String>>) -> Vec<HashMap<String, String>>{
    noun(nodes).iter().filter(|node| node["pos1"] == "サ変接続").map(|hm| hm.clone()).collect()
}

fn main() {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt".to_string();
    let neco: Vec<String> = NLP100::get(url).split("\n").filter(|f| f.ne(&"")).map(|m| m.to_string()).collect();
    let mut mecabu: Vec<HashMap<String, String>> = Vec::new();

    for line in neco {
        let mut tagger: Tagger = mecab::Tagger::new("");
        let nodes: Node = tagger.parse_to_node(line);

        for node in nodes.iter_next() {
            match node.stat as i32 {
                mecab::MECAB_BOS_NODE => (),
                mecab::MECAB_EOS_NODE => (),
                _ => {
                    mecabu.push(feature(&node));
                }
            }
        }
    }
    let word_histgram: HashMap<String, u64> = word_histgram(mecabu);
    let mut word_count: Vec<(String, u64)> = word_histgram.iter().map(|(word, count)| (word.to_owned(), count.to_owned())).collect();
    word_count.sort_by(|a, b| b.1.cmp(&a.1));
    let mut fg = Figure::new();
    let data: Vec<(String, u64)> = word_count.iter().map(|&(ref word, count)| (word.to_owned(), count.to_owned())).collect();
    let y: Vec<u64> = data.iter().map(|m| m.1).collect();
    let x = 0..(data.len() - 1);
    let labels = data.iter().enumerate().map(|(idx, &(ref word, _count))| Major(idx, Fix(word.clone())));

    let w = std::iter::repeat(0.5f32);

    fg.axes2d().set_title("頻度の高い単語", &[])
        .set_x_label("Num", &[])
        .set_y_label("Kinds", &[])
        .set_x_log(Some(10.0))
        .set_y_log(Some(10.0))
        .points(x, y, &[]);
    fg.set_terminal("encoding", "utf8");
    fg.set_terminal("png", "word-histgram.png");
    fg.show();
}
