fn read_line<T>() -> Vec<T>
where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim().split_whitespace().map(|c| T::from_str(c).unwrap()).collect()
}

fn f(x: i32, y: i32, z: i32, target: i32) -> i32 {
    let mut n = 0;
    for i in 0..(x+1) {
        for j in 0..(y+1) {
            for k in 0..(z+1) {
                let f = i * 500 + j * 100 + k * 50;
                if f == target {
                    n = n + 1;
                }
            }
        }
    }
    n
}

fn main() {
    let x: Vec<i32> = read_line();
    let y: Vec<i32> = read_line();
    let z: Vec<i32> = read_line();
    let w: Vec<i32> = read_line();
    let r = f(x[0], y[0], z[0], w[0]);
    println!("{}", r);
}
