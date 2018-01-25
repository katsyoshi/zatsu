use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate regex;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let file = File::open(path).unwrap();
    let re = Regex::new(r"\W+").unwrap();
    let hs = BufReader::new(file).lines().map(|m|{
        let l = m.unwrap().clone();
        re.split(&l).next().unwrap().to_string()
    }).collect::<HashSet<_>>();

    for s in hs {
        println!("{}", s);
    }

}
