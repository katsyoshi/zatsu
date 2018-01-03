fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    let first_length = first_word_length(&s);
    let second_length = second_word_length(&s);

    let hello = &s[0..5];
    let world = &s[6..11];

    // s.clear();

    println!("string: {}, first: {}", s, word);
    println!("hello: {}, world: {}", hello, world);
    println!("first: {}, second: {}", second_length.0, second_length.1);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_length(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn second_word_length(s: &str) -> (usize, usize) {
    let l = first_word_length(&s);
    (l, s.len() - l)
}
