extern crate regex;
use regex::Regex;

pub struct NLP100 {
    pub origin: String,
    pub words: Vec<String>,
    pub chars: Vec<String>,
}

impl NLP100 {
    pub fn new(script: &str) -> NLP100 {
        let chars = script.chars().map(|m| m.to_string()).collect::<Vec<String>>();
        let words = Regex::new(r"\W+").unwrap().split(script).map(|m| m.to_string()).collect::<Vec<String>>();
        let origin: String = script.to_string();

        NLP100 {
            words,
            chars,
            origin,
        }
    }

    pub fn ngram(self, size: u8, t: bool) -> Vec<String> {
        let mut val: Vec<String> = Vec::new();
        let mut i = 0;
        let (v, j) = if t { (self.words, " ") } else { (self.chars, "") };
        loop {
            let w = i + size as usize;
            if w > v.len() { break; }
            val.push(v[i..w].join(j));
            i += 1;
        }
        val
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

    #[test]
    fn bigram() {
        let nlp100 = NLP100::new("hello");
        assert_eq!(nlp100.ngram(2, false), vec!["he", "el", "ll", "lo"]);
    }

    #[test]
    fn trigram() {
        let nlp100 = NLP100::new("hello");
        assert_eq!(nlp100.ngram(3, false), vec!["hel", "ell", "llo"]);
    }

    fn setup() -> NLP100 {
        NLP100::new("hello")
    }
}
