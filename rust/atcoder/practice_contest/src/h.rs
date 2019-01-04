use std::collections::HashSet;

fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn main() {
    let x: Vec<i32> = read_line();
    let mut s: HashSet<i32> = HashSet::new();
    for _ in 0..x[0] {
        let v: Vec<i32> = read_line();
        s.insert(v[0]);
    }

    println!("{}", s.len());
}
