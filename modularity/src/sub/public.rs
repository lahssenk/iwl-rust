pub fn public() {
    // made visible to siblings in mod.rs
    println!("invoke super::private::public() from sub::public::public()");
    super::private::public();

    // super::hidden not visible

    // sibling fn is visible same module
    println!("invoke private() from sub::public::public()");
    private()
}

fn private() {
    println!("sub::public::private()");
}
