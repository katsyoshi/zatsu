fn main() {
    let dimension = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(dimension)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
