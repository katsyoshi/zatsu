fn sort(words: &mut Vec<&str>) {
    words.sort_by(|w, n| (w.to_lowercase().chars().collect::<Vec<char>>()[0] as u8).cmp(&(n.to_lowercase().chars().collect::<Vec<char>>()[0] as u8)));
    println!("{:?}", words);
}

fn main() {
    let mut s: Vec<&str> = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.".split(' ').collect();
    sort(&mut s);
}
