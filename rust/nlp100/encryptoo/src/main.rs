use std::ascii::AsciiExt;

fn cipher(src: &str) -> String {
    let chars = src.chars().collect::<Vec<char>>();
    let mut result: String = String::new();
    for c in chars {
        let s = if c.is_ascii() {
            let var: u8 = c as u8;
            match var {
                97 ... 122 => (219 - (var)) as char,
                _ => c,
            }
        } else {
            c
        };
        result.push(s);
    }
    result
}

fn main() {
    println!("{}", cipher("Today is fine."));
    println!("{}", cipher(&cipher("Today is fine.")));
}
