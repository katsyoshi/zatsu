extern crate nlp100;
extern crate regex;
use nlp100::NLP100;

fn sort(words: &mut Vec<&str>) {
    words.sort_by(|w, n| (w.to_lowercase().chars().collect::<Vec<char>>()[0] as u8).cmp(&(n.to_lowercase().chars().collect::<Vec<char>>()[0] as u8)));
    println!("{:?}", words);
}

fn main() {
    let nlp100 = NLP100::new("Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.");
    println!("{:?}", nlp100.char_count_list());
}
