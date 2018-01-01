fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    another_function();
    another_function_num(5);
    another_function_xy(5, 5);
    another_function_xy(x, y);
}

fn another_function() {
    println!("Another Function!");
}

fn another_function_num(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_xy(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
