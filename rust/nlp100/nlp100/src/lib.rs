pub struct NLP100 {
    pub origin: String,
    pub words: Vec<String>,
    pub chars: Vec<String>,
}

impl NLP100 {
    pub fn new(script: &str) -> NLP100 {
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
    fn origin() {
        let nlp100 = setup();
        assert_eq!(nlp100.origin, "hello");
    }

    #[test]
    fn chars() {
        let nlp100 = setup();
        assert_eq!(nlp100.chars, ["h", "e", "l", "l", "o"].to_vec());
    }

    #[test]
    fn words() {
        let nlp100 = setup();
        assert_eq!(nlp100.words, ["hello"].to_vec());
    }

    #[test]
    fn count_words(){
        let nlp100 = NLP100::new("h, l, l,o");
        assert_eq!(nlp100.words.len(), 4 as usize);
    }

    fn setup() -> NLP100 {
        NLP100::new("hello")
    }
}
