pub fn public() {
    // made visible to siblings in mod.rs
    super::private::public();

    // super::hidden not visible

    // sibling fn is visible same module
    private()
}

fn private() {
    println!("hello");
}
