fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn f(v: &mut Vec<i32>)  -> i32 {
    let mut a = 0;
    let mut b = 0;
    v.sort();
    v.reverse();
    for x in v.chunks(2) {
        a = a + x.get(0).unwrap_or(&0);
        b = b + x.get(1).unwrap_or(&0);
    }
    a - b
}

fn main() {
    let _: Vec<i32> = read_line();
    let mut x: Vec<i32> = read_line();

    let r = f(&mut x);
    println!("{}", r);
}
