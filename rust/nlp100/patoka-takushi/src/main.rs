fn concat(f: Vec<char>, s: Vec<char>) -> String {
    f.iter().zip(s).map(|(x, y)| format!("{}{}", x, y)).collect::<String>()
}

fn main() {
    let p: Vec<char> = "パトカー".chars().collect();
    let t: Vec<char> = "タクシー".chars().collect();
    println!("{}", concat(p, t));
}
