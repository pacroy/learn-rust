fn main() {
    println!("===== types =====");
    types();
    println!("===== conversion =====");
    conversion();
    println!("===== constants =====");
    constants();
    println!("===== arrays =====");
    arrays();
}

fn types() {
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false); // tuple - fixed sequences of values on the stack
    // slices - a collection of similar elements with length known at runtime
    let sentence = "hello world!"; // str (string slice) - text with a length known at runtime
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}

fn conversion() {
    // Rust requires explicitness when it comes to numeric types.
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

fn constants() {
    // constants directly replace the text identifier where they are used with their value at compile time.
    // Unlike variables, constants must always have explicit types.
    // Constant names are always in SCREAMING_SNAKE_CASE.
    const PI: f32 = 3.14159;

    println!(
        "To make an apple {} from scratch, you must first create a universe.",
        PI
    );
}

fn arrays() {
    // An array is a fixed length (known at compile time) collection of data elements all of the same type
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}