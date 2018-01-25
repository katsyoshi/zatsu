use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

enum Value {
    USIZE(usize),
    NONE(()),
}

fn main() {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let file = File::open(&args[0]).unwrap();
    let mut words: HashMap<String, usize> = HashMap::new();
    for m in BufReader::new(&file).lines() {
        let w = m.unwrap().to_string().split("\t").next().unwrap().to_string();
        let v = match words.get(&w) {
            None => 1,
            Some(n) => n + 1,
        };
        words.insert(w, v);
    }

    let mut vars: Vec<(&String, &usize)> = words.iter().collect();
    vars.sort_by(|a, b| b.1.cmp(a.1));
    for (w, v) in vars {println!("{}: {}", w, v);}

}
