// Function names are always in snake_case.

// if you define a function, the data it accepts are called parameters. 
// If you call that function and pass data to it, then it's called arguments.

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    // If you just want to return an expression, you can drop the return keyword and the semicolon at the end
    x - y
}

// Functions can return multiple values by returning a tuple of values.
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

// An empty tuple is represented by ().
fn make_nothing() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn main() {
    println!("===== basics =====");
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));

    println!("===== return a tuple of values =====");
    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    println!("===== returning nothing =====");
    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
}

