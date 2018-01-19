fn concat(t: (Vec<char>, Vec<char>)) {
    let (f, s) = t;
    let mut r = String::new();

    for (x, y) in f.iter().zip(s.iter()) {
        r.push_str(&format!("{}{}", x, y));
    }
    println!("{}", r);
}

fn main() {
    let p: Vec<char> = String::from("パトカー").chars().collect();
    let t: Vec<char> = String::from("タクシー").chars().collect();
    let f = (p, t);
    concat(f);
}
