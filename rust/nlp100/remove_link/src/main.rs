use std::collections::HashMap;
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
    let re = Regex::new(r"(?ms)(?:^\{\{基礎情報.*?$)(?P<dict>.+?)(?:^\}\}$)").unwrap();
    let dic = Regex::new(r"(?:^\|)(?P<key>.+?)\s*=\s*(?P<val>.+)").unwrap();
    let key = Regex::new(r"[^\^\|](?P<val>.+)").unwrap();
    let strong = Regex::new(r"'{3}").unwrap();
    let link = Regex::new(r"(?:\[{1,2})|(?:\]{1,2})").unwrap();

    for l in lines {
        let v: Value = serde_json::from_str(&l.unwrap()).unwrap();
        let text = &v["text"].as_str().unwrap();
        let title = &v["title"].as_str().unwrap().to_string();
        let mut results: HashMap<String, String> = HashMap::new();
        if keywords.contains(title) {
            let c = re.captures(text).unwrap();
            let mut prev_key = String::new();
            for line in c["dict"].split("\n") {
                if dic.is_match(line) {
                    let caps = dic.captures(line).unwrap();
                    prev_key = caps["key"].to_string();
                    let st = strong.replace_all(&caps["val"], "").to_string();
                    let val = link.replace_all(&st, "").to_string();
                    results.insert(prev_key.clone(), val);
                } else if key.is_match(line) {
                    let caps = key.captures(line).unwrap();
                    let pk = prev_key.clone();
                    if let Some(v) = results.get_mut(&pk) {
                        let st = strong.replace(&caps["val"].to_string(), "").to_string();
                        let val = link.replace_all(&st, "").to_string();
                        *v = vec![v.to_string(), val].join("");
                    }
                }
            }
            for (k, v) in results {
                println!("{}: {}", k, v);
            }
        }
    }
}

