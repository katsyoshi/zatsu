use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::io::prelude::*;

fn main() {
    let mut args = env::args();

    args.next();
    for path in args {
        let f = path.clone().replace(".txt", "");
        let s = path.clone().replace(".txt", "");
        let file = File::open(path).unwrap();
        let br = BufReader::new(file);
        let mut first_column = BufWriter::new(File::create(format!("{}_col1.txt", f)).unwrap());
        let mut second_column = BufWriter::new(File::create(format!("{}_col2.txt", s)).unwrap());

        for line in br.lines() {
            let words = line.unwrap().split("\t").map(|m| m.to_string()).collect::<Vec<String>>();
            first_column.write(format!("{}\n", words[0]).as_bytes()).unwrap();
            second_column.write(format!("{}\n", words[1]).as_bytes()).unwrap();
        }
    }
}
