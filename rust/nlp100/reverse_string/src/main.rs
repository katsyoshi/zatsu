use std::env;

fn reverse(args: env::Args) -> Result <(), ()> {
    Ok(
        for string in args {
            println!("{}", string.chars().rev().collect::<String>());
        }
    )
}

fn main() {
    reverse(env::args());
}
