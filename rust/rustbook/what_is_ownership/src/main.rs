fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(s);
    let x = 5;
    makes_copy(x);

    println!("x value is {}", x);


    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}, s3 = {}", s1, s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of \"{}\" is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

