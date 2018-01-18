extern crate nlp100;
extern crate regex;
use regex::Regex;
use nlp100::NLP100;

fn count(word: &str) -> u8 {
    Regex::new(r"\W+").unwrap().split(word).filter(|f| *f.to_string() != String::from("")).collect().len()
}

fn count_list(words: Vec<&str>) -> Vec<u8> {
    words.iter().map(|w| count(w)).collect::<Vec<u8>>()
}

fn sort(words: &mut Vec<&str>) {
    words.sort_by(|w, n| (w.to_lowercase().chars().collect::<Vec<char>>()[0] as u8).cmp(&(n.to_lowercase().chars().collect::<Vec<char>>()[0] as u8)));
    println!("{:?}", words);
}

fn main() {
    let s: Vec<&str> = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".split(' ').collect();
    println!("{:?}", count_list(s));
}
