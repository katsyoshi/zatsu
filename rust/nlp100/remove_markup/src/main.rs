use std::collections::HashMap;
use std::env;

extern crate nlp100;
extern crate regex;
extern crate serde_json;

use nlp100::NLP100;
use serde_json::Value;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let keywords = &args[1..];
    let lines = NLP100::read_gzip(path.to_string());
    let re = Regex::new(r"(?ms)(?:^\{\{基礎情報.*?$)(?P<dict>.+?)(?:^\}\}$)").unwrap();
    let dic = Regex::new(r"(?:^\|)(?P<key>.+?)\s*=\s*(?P<val>.+)").unwrap();
    let key = Regex::new(r"[^\^\|](?P<val>.+)").unwrap();
    let strong = Regex::new(r"'{2,5}").unwrap();
    let link = Regex::new(r"(?:\[{1,2})|(?:\]{1,2})").unwrap();
    let lang = Regex::new(r"(?:\{\{lang\|.+?\|)(?P<lang>.+?)(?:\}\})").unwrap();
    let markup = Regex::new(r"</*.+?>").unwrap();

    for l in lines {
        let v: Value = match serde_json::from_str(&l.as_str()) {
            Ok(x) => x,
            Err(_) => { continue; },
        };
        let text: String = match v["text"].as_str() {
            Some(x) => x.to_string(),
            None => { continue; },
        };
        let title: String = match v["title"].as_str() {
            Some(x) => x.to_string(),
            None => { continue; },
        };

        let mut results: HashMap<String, String> = HashMap::new();
        if keywords.contains(&title) {
            let c = re.captures(&text).unwrap();
            let mut prev_key = String::new();
            for line in c["dict"].split("\n") {
                if dic.is_match(line) {
                    let caps = dic.captures(line).unwrap();
                    prev_key = caps["key"].to_string();
                    results.insert(prev_key.clone(), caps["val"].to_string());
                } else if key.is_match(line) {
                    let caps = key.captures(line).unwrap();
                    let pk = prev_key.clone();
                    if let Some(v) = results.get_mut(&pk) {
                        *v = vec![v.to_string(), caps["val"].to_string()].join("");
                    }
                }
            }
            for (k, v) in results {
                let strong: String = strong.replace_all(&v, " ").trim().to_string();
                let link: String = link.replace_all(&strong, " ").trim().to_string();
                let markup: String = markup.replace_all(&link, " ").trim().to_string();
                let val: String = lang.replace_all(&markup, " $lang").trim().to_string();
                println!("{}: {}", k, val);
            }
        }
    }
}
