fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13"; // loop can break to return a value.
        }
    };
    println!("from loop: {}", v);
}
