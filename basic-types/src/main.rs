fn main() {
    // This is a comment

    // In rust, statements end with ';'

    // define a constant 8 bits unsigned integer
    const NUM: u8 = 1;

    // print a value using the println macro (macros end with '!')
    println!("NUM: {}", NUM);
    // print with debug
    println!("NUM: {:?}", NUM);

    // print using variable expansion
    println!("NUM: {NUM}");

    // declare an immutable string slice
    let _string_slice = "string slice";

    // declare an Owned string buffer. String type is stored on heap, is owned
    // and can grow.
    // NOTE: rust compiler will complain if you declare a var you don't use,
    // unless you prefix it with _
    let _owned_string = String::from("owned string");

    // create dynamic owned String buffer with the format! macro
    let _formatted_string = format!("x = {}, y = {y}", 10, y = 30);

    println!(
        "strings: owned={_owned_string:?}, slice={_string_slice:?}, formatted={_formatted_string:?}"
    );

    // unsigned integers
    let _unsigned8: u8 = 1;
    let _unsigned16: u16 = 1;
    let _unsigned32: u32 = 1;
    let _unsigned64: u64 = 1;
    let _unsigned128: u128 = 1;

    println!("unsigned: {_unsigned8:?}, {_unsigned16:?}, {_unsigned32:?}, {_unsigned64:?}, {_unsigned128:?}");

    // signed integers
    let _signed8: i8 = 1;
    let _signed16: i16 = 1;
    let _signed32: i32 = 1;
    let _signed64: i64 = 1;
    let _signed128: i128 = 1;

    println!("signed: {_signed8:?}, {_signed16:?}, {_signed32:?}, {_signed64:?}, {_signed128:?}");

    // booleans
    let _bool = true;

    // floats
    let _float32: f32 = 1.23;
    let _float64: f64 = 1.23;

    println!("floats: {_float32:?}, {_float64:?}");
}
