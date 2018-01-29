use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate serde_json;
use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let keywords = &args[1..];
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    for l in lines {
        let v: Value = serde_json::from_str(&l.unwrap()).unwrap();
        let country = v["title"].to_string().replace("\"","");

        if keywords.contains(&country) {
            println!("{}", v["text"]);
         }
    }
}
