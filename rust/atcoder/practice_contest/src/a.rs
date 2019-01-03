use std::io::{self, Read};
fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}
fn main() {
    let x: Vec<i32> = read_line();
    let y: Vec<i32> = read_line();
    let last: Vec<String> = read_line();
    let result = x[0] + y[0] + y[1];
    println!("{} {}", result, last[0]);
}
