fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn f(x: i32, y: i32) -> (i32, i32, i32) {
    let i = x + 1;
    for m in 0..i {
        for g in 0..(i - m) {
            let s = x - m - g;
            let k = m * 10000 + g * 5000 + s * 1000;
            if k == y { return (m, g, s); }
        }
    }
    (-1, -1, -1)
}

fn main() {
    let v: Vec<i32> = read_line();
    let (x, y, z) = f(v[0], v[1]);
    println!("{} {} {}", x, y, z);
}
