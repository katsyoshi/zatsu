fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn f(v: &Vec<(i32, i32, i32)>) -> &str {
    if v.iter().any(|&(t, i, j)| (i + j) > t || (i + j + t) % 2 == 1) {
        "No"
    } else {
        "Yes"
    }
}

fn main() {
    let n: Vec<i32> = read_line();
    let mut a = Vec::<(i32, i32, i32)>::new();

    for _ in 0..n[0] {
        let v: Vec<i32> = read_line();
        a.push((v[0], v[1], v[2]));
    }
    println!("{}", f(&a));
}
