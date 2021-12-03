// Rust has a built in generic enum called Result that allows us to return a value that has the possibility of failing.
fn do_something_that_might_fail(i:i32) -> Result<f32,String> {  // our generics type has multiple parameterized types separated by a comma.
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// Main returns no value, but could return an error!
fn main() -> Result<(), String> {
    let result = do_something_that_might_fail(12);

    // match lets us deconstruct Result elegantly and ensure we handle all cases!
    match result {
        Ok(v) => {
            println!("found {}", v);
            return Ok(());
        },
        Err(e) => {
            println!("Error: {}",e);

            // return a new error from main that said what happened!
            return Err(String::from("something went wrong in main!"));
        },
    }
}
