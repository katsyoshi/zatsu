fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn main() {
    let x: Vec<i64> = read_line();
    let result = if x[0] < x[1] { x[1] } else { x[0] };
    println!("{}", result);
}
