// Conditions don't have parentheses!
// All your usual relational and logical operators still work: ==, !=, <, >, <=, >=, !, ||, &&.
fn main() {
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}