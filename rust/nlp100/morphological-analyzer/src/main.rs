extern crate mecab;
extern crate nlp100;

use mecab::Node;
use mecab::Tagger;
use nlp100::NLP100;

fn main() {
    let url = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/neko.txt".to_string();
    let neco = NLP100::get(url);
    let mut tagger: Tagger = mecab::Tagger::new("");
    let nodes: Node = tagger.parse_to_node(neco);

    for node in nodes.iter_next() {
        if node.length > 0 {
            println!("{}: {}", &(node.surface)[..node.length as usize], node.feature);
        }
    }
}
