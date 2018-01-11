use std::env;

fn concat(mut args: env::Args) {
    args.next();
    let a = args.next().unwrap().chars().collect::<Vec<char>>();
    let b = args.next().unwrap().chars().collect::<Vec<char>>();
    let l = if a.len() > b.len() {
        a.len()
    } else {
        b.len()
    };

    let mut word = String::new();

    for x in 0..l {
        word.push(a[x]);
        word.push(b[x]);
    }
    println!("{}", word);
}

fn main() {
    concat(env::args());
}
