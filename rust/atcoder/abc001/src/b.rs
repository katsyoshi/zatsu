fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn km_vv(m: i32) -> i32 {
    if m < 100 { 0 }
    else if 100 <= m && m <= 5000 { m / 100 }
    else if 6000 <= m && m <= 30000 { m / 1000 + 50 }
    else if 35000 <= m && m <= 70000 { ((m / 1000) - 30) / 5 + 80 }
    else if 70000 < m { 89 }
    else { -1 }
}

fn main() {
    let x: Vec<i32> = read_line();
    let result = km_vv(x[0]);
    println!("{:02}", result);
}
