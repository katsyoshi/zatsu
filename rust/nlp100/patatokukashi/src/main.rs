fn main() {
    let chars = "パタトクカシーー".chars().collect::<Vec<char>>();
    let mut s = String::new();
    for (_i, c) in chars.iter().enumerate().filter(|&(i, _c)| i % 2 == 0 ) { s.push(*c); }
    println!("{}", s);
}
