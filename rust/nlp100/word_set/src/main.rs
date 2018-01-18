use std::collections::HashSet;
extern crate nlp100;
use nlp100::NLP100;

fn main() {
    let mut s1: HashSet<String> = HashSet::new();
    let paradise = NLP100::new("parapraparadise");
    for bi in paradise.ngram(2, false) { s1.insert(bi); }
    let mut s2: HashSet<String> = HashSet::new();
    let paragraph = NLP100::new("paragraph");
    for bi in paragraph.ngram(2, false) { s2.insert(bi); }

    println!("===UNION===");
    for x in s1.union(&s2) {
        println!("{}", x);
    }

    println!("\n===DIFF===");
    for x in s1.difference(&s2) {
        println!("{}", x);
    }

    println!("\n===INCLUDE===");
    let se = "se";
    println!("s1: {}", s1.contains(se));
    println!("s2: {}", s2.contains(se));
}
