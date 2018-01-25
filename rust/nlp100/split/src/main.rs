use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let path = &args[0].to_string();
    let file = File::open(path).unwrap();
    let count: usize = BufReader::new(file).lines().count();
    let div: usize = (&args[1]).to_string().parse().unwrap();
    let file = File::open(path).unwrap();
    let mut br = BufReader::new(file).lines();

    let t = (count as f64/ div as f64).ceil() as usize;

    for x in 1 .. div + 1 {
        let l = br.by_ref();
        let file = File::create(format!("{}.txt",x)).unwrap();
        let mut bw = BufWriter::new(file);
        for y in l.take(t).map(|m| m.unwrap().to_string()).collect::<Vec<String>>() {
            let a = format!("{}\n", y);
            bw.write(a.as_bytes()).unwrap();
            println!("{}: {}", x, y);
        }
    }
}
