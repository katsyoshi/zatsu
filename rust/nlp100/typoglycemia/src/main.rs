extern crate rand;
use rand::Rng;

fn words(src: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for s in src.split(' ').collect::<Vec<&str>>() {
        let mut chars = s.chars().collect::<Vec<char>>();
        if chars.len() < 3 {
            result.push(s.to_string());
        } else {
            let last_index = chars.len() - 1;
            let first_char = chars[0];
            let last_char = chars[last_index];

            let rand_chars = &mut chars[1..last_index];
            shuffle(rand_chars);
            let mut rand_string = String::new();
            for c in rand_chars { rand_string.push(*c) }
            result.push(format!("{}{}{}", first_char, rand_string, last_char));
        }
    }
    result
}

fn shuffle(chars: &mut [char]) {
    rand::thread_rng().shuffle(chars);
}

fn main() {
    let paragraph = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .";

    for w in words(paragraph) {
        println!("{}", w);
    }
}
