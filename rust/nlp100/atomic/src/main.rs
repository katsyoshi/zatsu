use std::collections::HashMap;

fn main() {
    let atomic_words: Vec<&str> = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.".split(' ').collect();
    let mut atomic_table = HashMap::new();
    for (i, a) in atomic_words.iter().enumerate() {
        let chars = a.chars().collect::<Vec<char>>();
        let c = chars[0].to_string();
        let he = chars[0..2].iter().cloned().collect::<String>();
        let r = match i {
            0 => c,
            4...8 => c,
            14...15 => c,
            18 => c,
            _ => he,
        };
        atomic_table.insert(r, i);
    }

    for (k, v) in &atomic_table {
        println!("{}: {}", k, v);
    }
}
