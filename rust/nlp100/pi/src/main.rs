fn count(word: &str) -> usize {
    word.chars().count()
}

fn count_list(words: Vec<&str>) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::new();
    for w in words {
        v.push(count(w));
    }
    v
}

fn sort(words: &mut Vec<&str>) {
    words.sort_by(|w, n| (w.to_lowercase().chars().collect::<Vec<char>>()[0] as u8).cmp(&(n.to_lowercase().chars().collect::<Vec<char>>()[0] as u8)));
    println!("{:?}", words);
}

fn main() {
    let mut s: Vec<&str> = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".split(' ').collect();
    println!("{:?}", count_list(s));
}
