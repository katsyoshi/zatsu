fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn f(n: i32, x: i32, y: i32)  -> i32 {
    let mut r = 0;
    for i in 0..(n+1) {
        let st = i.to_string();
        let mut sum = 0;
        for s in st.chars() {
            let f = (s.to_string()).parse::<i32>().unwrap_or(0);
            sum += f;
        }
        if x <= sum && sum <= y {
            r += i
        }
    }
    r
}

fn main() {
    let x: Vec<i32> = read_line();

    let r = f(x[0], x[1], x[2]);
    println!("{}", r);
}
