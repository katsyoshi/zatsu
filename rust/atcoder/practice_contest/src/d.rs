fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn c<'a>(v: &'a Vec<i32>) -> bool {
    v.iter().filter(|&i| i % 2 == 1).count() == 0
}

fn f<'a>(v: &'a Vec<i32>, n: &'a mut i32) -> Vec<i32> {
    if c(v) {
        *n = *n + 1;
        let k = v.iter().map(|&i| i / 2).collect::<Vec<i32>>();
        f(&k, n)
    } else { vec!() }
}

fn main() {
    let _: Vec<usize> = read_line();
    let x: Vec<i32> = read_line();
    let mut y: i32 = 0;
    f(&x, &mut y);
    println!("{}", y);
}
