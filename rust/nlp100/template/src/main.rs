fn string_template(x: i8, y: &str, z: f32) -> String {
    format!("{}時の{}は{}", x, y, z)
}

fn main() {
    let string = string_template(12, "気温", 22.5);
    println!("{}", string);
}
