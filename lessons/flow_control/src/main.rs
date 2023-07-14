fn main() {
    // blocks
    let n = 2;
    let x = { 1 + n };
    println!("x={x}");

    // this will panic if x!=3
    assert_eq!(x, 3);

    // if else expressions
    let a = true;
    if a {
        println!("a is true")
    } else {
        println!("a is false")
    }

    // doing the equivalent with match a expression
    match a {
        true => println!("a is true"),
        false => println!("a is false"),
    }

    let one = 1;
    match one {
        1 => println!("one is 1"),
        _ => println!("catch all fallback branch: one is {one}"),
    }

    // declare a Some variant of the Option type.
    let maybe_true = Some(true);

    // if let is useful for only assigning based on pattern matching and deconstruction
    if let Some(b) = maybe_true {
        println!("maybe_true option is {:?}", b)
    } else {
        println!("maybe_true option is None")
    }

    // while loop stops when the condition is not verified anymore
    let mut i = 0;
    while i < 3 {
        println!("i= {i}");
        i += i;
    }

    // while let repeatedly tests a value against a pattern
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    // the iterator will return None when it reached the end
    // which will be the stop condition in the wile let loop
    while let Some(x) = iter.next() {
        println!("x: {x}");
    }

    // for loops are pretty standard
    // create a vector
    let v = vec![10, 20, 30];
    // iterate over all elems of the vector iterator
    // invoking iter() so that v is not moved and we can reuse
    // it in next for loop
    for x in v.iter() {
        println!("x: {x}");
    }

    for (ind, val) in v.iter().enumerate() {
        println!("ind = {ind}, value = {val}");
    }

    // iterate over a mutable vector to replace values
    let mut v = vec![10, 20, 30];
    // iterate over all elems of the vector (implements iterator)
    for x in v.iter_mut() {
        *x += 1;
        println!("x: {x}");
    }

    // iterator over a range
    // step_by returns an iterator from the range
    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }

    // a loop expression has no break condition and
    // break must be invoked explicilty
    let mut x = 3;
    loop {
        x -= 1;

        if x == 0 {
            break;
        };

        if x == 3 {
            continue;
        }

        println!("x={x}");
    }

    // rewrite it with match
    let mut x = 3;
    'named_loop: loop {
        x -= 1;

        match x {
            3 => {
                continue;
            }
            0 => break 'named_loop,
            _ => {
                println!("{x}");
            }
        }
    }
}
