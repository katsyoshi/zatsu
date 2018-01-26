use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let file = File::open(&args[0]).unwrap();
    let takes = (&args[1]).to_string().parse::<usize>().unwrap();
    let br = BufReader::new(&file).lines();
    let skips = br.count() - takes;
    let file = File::open(&args[0]).unwrap();
    let lines = BufReader::new(&file).lines().skip(skips);

    for line in lines {
        println!("{}", line.unwrap());
    }
}

