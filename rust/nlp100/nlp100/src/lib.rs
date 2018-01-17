pub struct NLP100 {
    origin: String,
    words: Vec<String>,
    chars: Vec<String>,
}

impl NLP100 {
    fn new(script: &str) -> NLP100 {
        let chars = script.chars().map(|m| m.to_string()).collect::<Vec<String>>();
        let words = script.split(' ').map(|m| m.to_string()).collect::<Vec<String>>();
        NLP100 {
            words,
            chars,
            origin: script.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use NLP100;
    #[test]
    fn it_works() {
        let nlp100 = NLP100::new("hello");
        assert_eq!(nlp100.origin, "hello");
        assert_eq!(nlp100.words, ["hello"].to_vec());
        assert_eq!(nlp100.chars, ["h", "e", "l", "l", "o"].to_vec());
    }
}
