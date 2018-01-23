use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let file = File::open(&args[0]).unwrap();

    let br = BufReader::new(file).lines().take((&args[1]).to_string().parse::<usize>().unwrap());

    for line in br.map(|m| m.unwrap().to_string()).collect::<Vec<String>>() {
        println!("{}", line);
    }
}
