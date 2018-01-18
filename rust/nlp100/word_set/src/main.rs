use std::collections::HashSet;
extern crate nlp100;
use nlp100::NLP100;

fn bigram(words: Vec<String>) -> Vec<Vec<String>>{
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut itr = words.iter();
    let mut now = itr.next().unwrap().to_string();

    loop {
        let mut ngram: Vec<String> = Vec::new();
        ngram.push(now);
        match itr.next() {
            Some(s) => {
                now = s.to_string();
                ngram.push(s.to_string());
                result.push(ngram);
            },
            None => break,
        }
    }

    result
}

fn main() {
    let mut s1: HashSet<Vec<String>> = HashSet::new();
    let paradise = NLP100::new("parapraparadise");
    for bi in bigram(paradise.chars) { s1.insert(bi); }
    let mut s2: HashSet<Vec<String>> = HashSet::new();
    let paragraph = NLP100::new("paragraph");
    for bi in bigram(paragraph.chars) { s2.insert(bi); }

    println!("===UNION===");
    for x in s1.union(&s2) {
        println!("{}{}", x[0], x[1]);
    }

    println!("\n===DIFF===");
    for x in s1.difference(&s2) {
        println!("{}{}", x[0], x[1]);
    }

    println!("\n===INCLUDE===");
    let se = vec!["s".to_string(), "e".to_string()];
    println!("s1: {}", s1.contains(&se));
    println!("s2: {}", s2.contains(&se));
}
