use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

pub struct NLP100 {
    pub origin: String,
    pub words: Vec<String>,
    pub chars: Vec<String>,
}

impl NLP100 {
    pub fn new(script: &str) -> NLP100 {
        let chars = script.chars().map(|m| m.to_string()).collect::<Vec<String>>();
        let words = Regex::new(r"\W+").unwrap().split(script).map(|m| m.to_string()).collect::<Vec<String>>();
        let origin: String = script.to_string();

        NLP100 {
            words,
            chars,
            origin,
        }
    }

    pub fn ngram(self, size: u8, t: bool) -> Vec<String> {
        let mut val: Vec<String> = Vec::new();
        let mut i = 0;
        let (v, j) = if t { (self.words, " ") } else { (self.chars, "") };
        loop {
            let w = i + size as usize;
            if w > v.len() { break; }
            val.push(v[i..w].join(j));
            i += 1;
        }
        val
    }

    pub fn char_count_list(self) -> Vec<u32> {
        self.words.iter().map(|v| v.len() as u32).collect()
    }

    pub fn chars_first_to(word: String, stop: usize) -> String{
        (word.chars().map(|c| c.to_string()).collect::<Vec<String>>()[0..stop]).join("")
    }

    pub fn words_first_to(self, stop: usize) -> Vec<String> {
        self.words.iter().map(|word| NLP100::chars_first_to(word.to_string(), stop)).collect::<Vec<String>>()
    }

    pub fn read_file(self, path: String) -> Vec<String> {
        let file = File::open(path).unwrap();
        BufReader::new(file).lines().map(|m| m.unwrap().to_string()).collect::<Vec<String>>()
    }
}


#[cfg(test)]
mod tests {
    use NLP100;

    #[test]
    fn origin() {
        let nlp100 = setup();
        assert_eq!(nlp100.origin, "hello");
    }

    #[test]
    fn chars() {
        let nlp100 = setup();
        assert_eq!(nlp100.chars, vec!["h", "e", "l", "l", "o"]);
    }

    #[test]
    fn words() {
        let nlp100 = setup();
        assert_eq!(nlp100.words, vec!["hello"]);
    }

    #[test]
    fn count_words(){
        let nlp100 = NLP100::new("h, l, l,o").words;
        assert_eq!(nlp100.len(), 4 as usize);
        assert_eq!(nlp100, vec!["h", "l", "l", "o"]);
    }

    #[test]
    fn bigram() {
        let nlp100 = NLP100::new("hello");
        assert_eq!(nlp100.ngram(2, false), vec!["he", "el", "ll", "lo"]);
    }

    #[test]
    fn trigram() {
        let nlp100 = NLP100::new("hello");
        assert_eq!(nlp100.ngram(3, false), vec!["hel", "ell", "llo"]);
    }

    #[test]
    fn word_cound() {
        let nlp100 = NLP100::new("hello, world!");
        assert_eq!(nlp100.char_count_list(), vec![5, 5]);
    }

    #[test]
    fn words_first_to_one() {
        let nlp100 = NLP100::new("hello, world!!!");
        assert_eq!(nlp100.words_first_to(1), vec!["h", "w"]);
    }

    #[test]
    fn words_first_to_two() {
        let nlp100 = NLP100::new("hello, world!!!");
        assert_eq!(nlp100.words_first_to(2), vec!["he", "wo"]);
    }

    #[test]
    fn read_file() {
        let s = setup();
        let line = s.read_file("hightemp.txt".to_string()).len();
        assert_eq!(line, 24);
    }

    fn setup() -> NLP100 {
        NLP100::new("hello")
    }
}
