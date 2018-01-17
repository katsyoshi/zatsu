fn count(word: &str) -> u8 {
    let chars: Vec<char> = word.chars().collect();
    let mut count = 0;
    for c in chars {
        let var: u8 = c as u8;
        match var {
            48 ... 58 => { count = count + 1; },
            65 ... 91 => { count = count + 1; },
            96 ... 123 => { count = count + 1; },
            _ => (),
        }
    }
    count
}

fn count_list(words: Vec<&str>) -> Vec<u8> {
    words.iter().map(|w| count(w)).collect::<Vec<u8>>()
}

fn sort(words: &mut Vec<&str>) {
    words.sort_by(|w, n| (w.to_lowercase().chars().collect::<Vec<char>>()[0] as u8).cmp(&(n.to_lowercase().chars().collect::<Vec<char>>()[0] as u8)));
    println!("{:?}", words);
}

fn main() {
    let s: Vec<&str> = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".split(' ').collect();
    println!("{:?}", count_list(s));
}
