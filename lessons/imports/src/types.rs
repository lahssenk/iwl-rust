// add symbols from external crate serde into scope
use serde::{Deserialize, Serialize};

// derive traits from serde crate
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MyStruct {
    pub age: usize,
    pub name: String,
}
