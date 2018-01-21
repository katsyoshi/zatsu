use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let mut args = env::args();

    args.next();
    for path in args {
        let f = File::open(path).unwrap();
        let br = BufReader::new(f);

        println!("{}", br.lines().count());
    }
}
