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
    let phrase = NLP100::new("I am an NLPer");
    println!("\n===word bi-gram");
    for word in bigram(phrase.words) {
        println!("{:?}", word);
    }

    println!("\n===charactor bi-gram");
    for word in bigram(phrase.chars) {
        println!("{:?}", word);
    }
}
