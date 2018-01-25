use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let file = File::open(path).unwrap();
    let mut val = BufReader::new(file).lines().map(|m| m.unwrap().split("\t").skip(2).next().unwrap().parse::<f64>().unwrap()).collect::<Vec<f64>>();

    val.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for v in val { println!("{}", v); }
}
