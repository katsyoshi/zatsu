fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn children(s: &String) -> String {
    s.chars().filter(|&c| c != 'a' && c != 'i' && c != 'u' && c != 'e' && c != 'o').collect()
}

fn main() {
    let x: Vec<String> = read_line();
    let result = children(&x[0]);
    println!("{}", result);
}
