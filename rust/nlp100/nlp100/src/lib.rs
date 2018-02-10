extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate libflate;
extern crate regex;
extern crate serde_json;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use libflate::gzip::Decoder;
use regex::Regex;
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use tokio_core::reactor::Core;

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

    pub fn chars_first_to(word: String, stop: usize) -> String {
        (word.chars().map(|c| c.to_string()).collect::<Vec<String>>()[0..stop]).join("")
    }

    pub fn words_first_to(self, stop: usize) -> Vec<String> {
        self.words.iter().map(|word| NLP100::chars_first_to(word.to_string(), stop)).collect()
    }

    fn open(path: String) -> File {
        File::open(path).unwrap()
    }

    pub fn read_gzip(path: String) -> Vec<String> {
        let mut file = NLP100::open(path);
        let mut string = String::new();
        Decoder::new(&mut file).unwrap().read_to_string(&mut string).unwrap();
        string.split("\n").map(|m| m.to_string()).collect()
    }

    pub fn read(path: String) -> Vec<String> {
        BufReader::new(NLP100::open(path)).lines().map(|m| m.unwrap().to_string()).collect()
    }

    pub fn count(path: String) -> usize {
        BufReader::new(NLP100::open(path)).lines().count()
    }

    pub fn get(url: String) -> String {
        let mut core = Core::new().unwrap();
        let url = url.parse().unwrap();
        let handle = core.handle();
        let client = Client::configure().connector(HttpsConnector::new(4, &handle).unwrap()).build(&handle);
        let work = client.get(url).and_then(|res| {
            res.body().concat2().map(|chunk| {
                let v = chunk.to_vec();
                String::from_utf8_lossy(&v).to_string()
            })
        });

        match core.run(work) {
            Ok(v) => v,
            Err(e) => { panic!(e); },
        }
    }

    pub fn parse_json(json: String) -> Value {
        match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => { panic!(e); },
        }
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
        let line = NLP100::read(String::from("hightemp.txt"));
        assert_eq!(line[0], "高知県\t江川崎\t41\t2013-08-12");
    }

    #[test]
    fn count_line() {
        let line = NLP100::count(String::from("hightemp.txt"));
        assert_eq!(line, 24);
    }

    #[test]
    fn deflate() {
        let path = String::from("jawiki-country.json.gz");
        let json = NLP100::read_gzip(path);
        assert_eq!(55586, json[0].len());
    }

    #[test]
    fn parse_json() {
        let json = NLP100::parse_json(String::from("{ \"hello\": 10 }"));
        assert_eq!(json["hello"], 10);
    }

    #[test]
    fn get() {
        let html = NLP100::get(String::from("https://katsyoshi.org/nlp100.json"));
        assert_eq!(html, "");
    }

    fn setup() -> NLP100 {
        NLP100::new("hello")
    }
}
