extern crate nlp100;
use nlp100::NLP100;

fn main() {
    let nlp100 = NLP100::new("パタトクカシーー");
    let mut s = String::new();
    for (_i, c) in nlp100.chars.iter().enumerate().filter(|&(i, _c)| i % 2 == 0 ) { s += c; }
    println!("{}", s);
}
