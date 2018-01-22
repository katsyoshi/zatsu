use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::io::prelude::*;

fn main() {
    let mut args = env::args();
    if args.len() < 2 { panic!("col1.txt col2.txt"); }

    args.next();

    let first = args.next().unwrap();
    let second = args.next().unwrap();

    let fr = BufReader::new(File::open(first).unwrap()).lines().map(|m| m.unwrap().to_string()).collect::<Vec<String>>();

    let sr = BufReader::new(File::open(second).unwrap()).lines().map(|m| m.unwrap().to_string()).collect::<Vec<String>>();
    let mut merge_file =
        BufWriter::new(File::create("merge.txt".to_string()).unwrap());

    for (x, y) in fr.iter().zip(&sr) {
        merge_file.write(format!("{}\t{}\n", x, y).as_bytes()).unwrap();
    }

}
