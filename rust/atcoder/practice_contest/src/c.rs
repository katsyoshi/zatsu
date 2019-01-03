fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn char_count(s: &String) -> usize {
    s.chars().filter(|&c| c == '1').collect::<Vec<char>>().len()
}

fn main() {
    let x: Vec<String> = read_line();

    println!("{}", char_count(&x[0]));
}
