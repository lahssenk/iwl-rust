mod types;

fn main() {
    let object = types::MyStruct {
        age: 12,
        name: String::from("john"),
    };

    // Convert the object to a JSON string.
    let serialized = serde_json::to_string(&object).unwrap();

    println!("serialized = {}", serialized);

    // Convert the JSON string back to an instance of MyStruct.
    // use external crate serde_json symbols with full path
    let deserialized: types::MyStruct = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    assert_eq!(object, deserialized);
}
