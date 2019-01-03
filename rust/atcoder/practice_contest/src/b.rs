fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn main() {
    let x: Vec<i32> = read_line();

    let odd = (x[0] * x[1]) % 2 ;
    let result = if odd == 0 { "Even" } else { "Odd" };
    println!("{}", result);
}
