use std::panic;

fn main() {
    let input = 10;
    let res = might_fail(input);

    // clone the result to reuse it, then use match
    // to handle the variants of the Result enum
    match res.clone() {
        Ok(v) => println!("might_fail({input}): succeeded with {v}"),
        Err(s) => println!("might_fail({input}): failed with {s}"),
    }

    // using if let to deconstruct an OK case
    if let Ok(v) = res {
        println!("it was OK({v})");
    }

    // assert that the result was Ok(10)
    assert_eq!(Ok(10), res);

    // catch a panic.
    // By default, a panic will cause the stack to unwind. The unwinding can be caught with panic::catch_unwind
    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    assert!(result.is_err());

    // use the unwrap() shortcut to panic on Err instead of properly handling errors
    let res = bubble_error_up(1).unwrap();
    assert_eq!(res, 2);
}

// this function invokes might_fail, and uses the try operator (?)
// to bubble up the error and only keep the happy scenario (success)
fn bubble_error_up(i: usize) -> Result<usize, String> {
    let res = might_fail(i)?;
    Ok(res + 1)
}

// declare a function that might fail, meaning
// it returns a Result. The Result type is a
// Generic enum with two variants: Ok(R) and Err(E).
fn might_fail(i: usize) -> Result<usize, String> {
    match i % 2 == 0 {
        true => Ok(i),
        false => Err(String::from("i must be an odd number")),
    }
}
