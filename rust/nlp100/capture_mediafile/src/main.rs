use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate regex;
extern crate serde_json;

use serde_json::Value;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let keywords = &args[1..];
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    let re = Regex::new(r"\[\[(File|ファイル):(?P<filename>.+?)(?:\|.*)*(?:\|.*)*\]\]").unwrap();

    for l in lines {
        let v: Value = serde_json::from_str(&l.unwrap()).unwrap();

        let title = &v["title"].as_str().unwrap().to_string();
        if keywords.contains(title) {
            for content in v["text"].as_str().unwrap().to_string().split("\n").filter(|m| re.is_match(m)) {
                for caps in re.captures_iter(content) {
                    println!("filename: {}", &caps["filename"]);
                }
            }
        }
    }
}
