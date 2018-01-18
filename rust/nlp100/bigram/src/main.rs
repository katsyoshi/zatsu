extern crate nlp100;
use nlp100::NLP100;

fn main() {
    let phrase = NLP100::new("I am an NLPer");
    println!("\n===word bi-gram");
    for word in phrase.ngram(2, true) {
        println!("{}", word);
    }

    let phrase = NLP100::new("I am an NLPer");
    println!("\n===charactor bi-gram");
    for word in phrase.ngram(2, false) {
        println!("\"{}\"", word);
    }
}
