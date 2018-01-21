use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let mut args = env::args();
    args.next();
    for path in args {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line.unwrap().replace("\t", " "));
        }
    }
}
