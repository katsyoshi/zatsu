struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("email: {}", user1.email);
    println!("username: {}", user1.username);
    println!("active: {}", user1.active);
    println!("sigin count: {}", user1.sign_in_count);

    user1.email = String::from("anotheremail@example.com");

    println!("email: {}", user1.email);

    let user2 = build_user(String::from("hoge@example.com"), String::from("hoge"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
