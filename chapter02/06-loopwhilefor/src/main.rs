fn main() {
    println!("==== loop ====");
    try_loop();
    println!("==== while ====");
    try_while();
    println!("==== for ====");
    try_for();
}

fn try_loop() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break; // break will escape a loop
        }
    }
    println!("{}", x);
}

fn try_while() {
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    println!("x is {}", x);
}

fn try_for() {
    // Rust's for loop is a powerful upgrade. It iterates over values from any expression that evaluates into an iterator.

    for x in 0..5 { // The .. operator creates an iterator that generates numbers from a start number up to but not including an end number.
        println!("{}", x);
    }

    for x in 0..=5 { // The ..= operator creates an iterator that generates numbers from a start number up to and including an end number.
        println!("{}", x);
    }
}
