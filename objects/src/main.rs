fn main() {
    // all fields of an instance of a struct
    // must be instantiated when you construct it.
    let my_object = MyStruct {
        name: String::from("john"),
        age: 18,
    };

    println!(
        "MyStruct instance: {my_object:?}, name={}, age={}",
        my_object.name, my_object.age
    );

    // create a struct with default values
    let mut my_object = MyStruct::default();

    println!(
        "MyStruct instance: {my_object:?}, name={}, age={}",
        my_object.name, my_object.age
    );

    my_object.name = String::from("the name");
    my_object.age = 18;

    println!(
        "MyStruct instance: {my_object:?}, name={}, age={}",
        my_object.name, my_object.age
    );

    // instanciate an object partially by filling missing fields from another object
    let other_object = MyStruct {
        name: String::from("other name"),
        ..my_object
    };

    println!(
        "MyStruct instance: {other_object:?}, name={}, age={}",
        other_object.name, other_object.age
    );

    // original object left untouched
    println!(
        "MyStruct instance: {my_object:?}, name={}, age={}",
        my_object.name, my_object.age
    );
}

#[derive(Debug, Default)]
struct MyStruct {
    name: String,
    age: i8,
}
