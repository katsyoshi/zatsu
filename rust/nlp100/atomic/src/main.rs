extern crate nlp100;
use nlp100::NLP100;

fn hi(a: &str, c: usize) -> String {
    (&(a.chars().map(|v| v.to_string()).collect::<Vec<String>>())[0..c]).join("")
}

fn atomic_table(words: Vec<String>) -> Vec<String> {
    (0..).zip(words.iter()).map(|(i, a)| {
        format!("{}: {}", {
            match i {
                0 | 4...8 | 14...15 | 18 => hi(a,1),
                _ => hi(a, 2),
            }
        }, i + 1)
    }).collect::<Vec<String>>()
}

fn main() {
    let atomic = NLP100::new("Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.");

    for s in atomic_table(atomic.words) {println!("{}", s);}
}
