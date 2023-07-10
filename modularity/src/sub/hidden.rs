pub fn public() {
    println!("sub::hidden::public()");

    // does not have access to sub::private or sub::public because
    // it is missing from mod.rs
}
