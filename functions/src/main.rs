// declare a function named 'my_func'
// which accepts two arguments:
// - a is an unsigned integer
// -b is a reference to a string (string slice)
// It returns an unsigned integer, which will
// be the sum of a nd b if b can be parsed as usize.
// Panics if it can not be parsed.
fn my_func(a: usize, b: &str) -> usize {
    let parsed = b.parse::<usize>().unwrap();
    a + parsed
}

fn main() {
    // declare the values we'll use as arguments
    let a = 1;
    let b = String::from("2");
    // invoke our function
    let res = my_func(a, b.as_str());
    println!("res: {res}");
}
