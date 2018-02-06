use std::collections::HashMap;
use std::env;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate nlp100;
extern crate regex;
extern crate serde_json;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use nlp100::NLP100;
use serde_json::Value;
use regex::Regex;
use tokio_core::reactor::Core;

fn get_image(query: String) -> String {
    let re = Regex::new(r" ").unwrap();
    let uri = format!("https://en.wikipedia.org/w/api.php?action=query&titles=File:{}&format=json&prop=imageinfo&iiprop=url", re.replace_all(&query, "%20"));
    let uri = uri.parse().unwrap();
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let work = client.get(uri).and_then(|res| {
        res.body().concat2().and_then(move |body| {
            let v: Value = serde_json::from_slice(&body).unwrap();
            let page_ids = v["query"]["pages"].as_object().unwrap().keys().map(|m| m.to_string()).collect::<Vec<String>>();
            let imageinfo = &v["query"]["pages"][&page_ids[0]]["imageinfo"];
            let r = format!("{}", imageinfo[0]["url"].as_str().unwrap());
            Ok(r)
        })
    });
    core.run(work).unwrap()
}

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
            Ok(v) => v,
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
                match dic.captures(line) {
                    Some(caps) => {
                        prev_key = caps["key"].to_string();
                        results.insert(prev_key.clone(), caps["val"].to_string());
                    },
                    None => {
                        match key.captures(line) {
                            Some(caps) => {
                                let pk = prev_key.clone();
                                match results.get_mut(&pk) {
                                    Some(v) => {
                                        *v = vec![v.to_string(), caps["val"].to_string()].join("");
                                        ()
                                    },
                                    None => { continue; },
                                };
                            },
                            None => { continue; },
                        }
                    },
                }
            }
            for (k, v) in results {
                let strong: String = strong.replace_all(&v, " ").trim().to_string();
                let link: String = link.replace_all(&strong, " ").trim().to_string();
                let markup: String = markup.replace_all(&link, " ").trim().to_string();
                let val: String = if k == "国旗画像" {
                    get_image(v.to_string())
                } else {
                    lang.replace_all(&markup, " $lang").trim().to_string()
                };
                println!("{}: {}", k, val);
            }
        }
    }
}
