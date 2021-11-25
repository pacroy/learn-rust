fn main() {
    println!("===== basic =====");
    basic();
    println!("===== mutable =====");
    mutable();
}

fn basic() {
    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);
}

fn mutable() {
    // Mutable values are denoted with a `mut` keyword.
    //  mutable - the compiler will allow the variable to be written to and read from.
    //  immutable - the compiler will only allow the variable to be read from.
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}
