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
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
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

fn read_gzip(path: String) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut string = String::new();
    Decoder::new(&mut file).unwrap().read_to_string(&mut string).unwrap();
    string.split("\n").filter(|m| m.ne(&"")).map(|m| m.to_string()).collect::<Vec<String>>()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let keywords = &args[1..];
    let lines = read_gzip(path.to_string());
    let re = Regex::new(r"(?ms)(?:^\{\{基礎情報.*?$)(?P<dict>.+?)(?:^\}\}$)").unwrap();
    let bar_hat = Regex::new(r"(?ms)(?:^\|)").unwrap();
    let dict = Regex::new(r"(?P<key>.+?)\s*=\s*(?P<val>.+)").unwrap();
    let media = Regex::new(r"\[\[(File|ファイル):(?P<filename>.+?)(?:\|.*)*(?:\|.*)*\]\]").unwrap();
    let strong = Regex::new(r"'{2,5}").unwrap();
    let link = Regex::new(r"(?:\[{1,2})(?P<link>.+?)(?:\|.+)?(?:\]{1,2})").unwrap();
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

        if keywords.contains(&title) {
            let mut results: HashMap<String, String> = HashMap::new();
            let re: String = match re.captures(&text) {
                Some(caps) => caps["dict"].to_string(),
                None => {
                    println!("cannot capture dict!!");
                    continue;
                },
            };
            for line in bar_hat.split(&re).filter(|f| f.ne(&"")).map(|m| m.to_string()) {
                let line = line.replace("\n", "");
                let dict = match dict.captures(&line) {
                    Some(x) => x,
                    None => {continue;},
                };

                let val = dict["val"].to_string();
                let key = dict["key"].to_string();
                let file: String = media.replace_all(&val, "$filename").trim().to_string();
                let strong: String = strong.replace_all(&file, "").trim().to_string();
                let link: String = link.replace_all(&strong, "$link").trim().to_string();
                let markup: String = markup.replace_all(&link, "").trim().to_string();
                let val: String = match key.as_ref() {
                    "国旗画像" => get_image(markup.to_string()),
                    _ => lang.replace_all(&markup, "$lang").trim().to_string(),
                };
                results.insert(key, val);
            }
            for (k, v) in results {
                println!("{}: {}", k, v);
            }
        }
    }
}
