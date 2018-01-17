fn chars(phrase: String) -> Vec<String> {
    phrase.chars().collect::<Vec<char>>().iter().map(|m| m.to_string()).collect::<Vec<String>>()
}

fn words(phrase: String) -> Vec<String> {
    phrase.split(' ').collect::<Vec<&str>>().iter().map(|m| m.to_string()).collect::<Vec<String>>()
}

fn bigram(words: Vec<String>) -> Vec<Vec<String>>{
    let mut result: Vec<Vec<String>> = Vec::new();
    let mut itr = words.iter();
    let mut now = itr.next().unwrap().to_string();

    loop {
        let mut ngram: Vec<String> = Vec::new();
        ngram.push(now);
        match itr.next() {
            Some(s) => {
                now = s.to_string();
                ngram.push(s.to_string());
                result.push(ngram);
            },
            None => break,
        }
    }

    result
}

fn main() {
    let phrase = "I am an NLPer";
    println!("\n===word bi-gram");
    for word in bigram(words(phrase.to_string())) {
        println!("{:?}", word);
    }

    println!("\n===charactor bi-gram");
    for word in bigram(chars(phrase.to_string())) {
        println!("{:?}", word);
    }
}
