use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let file = File::open(&args[0]).unwrap();
    let takes = (&args[1]).to_string().parse::<usize>().unwrap();

    let br = BufReader::new(file).lines().map(|m| m.unwrap().to_string()).collect::<Vec<String>>();
    let rev = br.iter().rev().take(takes).map(|m| m.to_string()).collect::<Vec<String>>();

    for line in rev.iter().rev() {
        println!("{}", line);
    }
}

