fn main() {
    // enums in rust are a union
    // type that can hold multiple variants.
    // Enum variants can hold payloads/data.
    // In order to inspect which variant of
    // an enum a specific instance of the enum
    // type holds, it is necessary to do pattern matching.

    let enum_value = MyEnum::EmptyVariant;
    println!("enum_value: {enum_value:?}");

    let enum_value = MyEnum::IntVariant(1);
    println!("enum_value: {enum_value:?}");

    let enum_value = MyEnum::FloatVariant(2.5);
    println!("enum_value: {enum_value:?}");

    let enum_value = MyEnum::StringVariant("abc".into());
    println!("enum_value: {enum_value:?}");

    // by default pattern matching must be exhaustive, to check against all
    // possible variants
    match enum_value {
        MyEnum::EmptyVariant => println!("{enum_value:?} is an EmptyVariant"),
        MyEnum::IntVariant(_) => println!("{enum_value:?} is an IntVariant"),
        MyEnum::FloatVariant(_) => println!("{enum_value:?} is an FloatVariant"),
        MyEnum::StringVariant(_) => println!("{enum_value:?} is an StringVariant"),
    }
}

#[derive(Debug)]
enum MyEnum {
    EmptyVariant,
    IntVariant(i32),
    FloatVariant(f32),
    StringVariant(String),
}
