fn main() {
    // construct an instance explicitly
    let object = MyStruct { age: 10 };
    object.print_age();

    // or use the 'new' associated func
    let mut object = MyStruct::new(10);
    object.print_age();

    object.update_age(11);
    object.print_age();

    let object2 = object.consume();
    object2.print_age();

    // object has been consumed, and can not be used anymore
    // uncommenting the next line would fail to compile
    //  object.print_age();
}

struct MyStruct {
    age: usize,
}

// define the implementation (methods and associated functions)
// for our type. Rust separate data from implementation and manipulation.
impl MyStruct {
    // define an associated function (attached to the type, not an instance of it).
    // useful for scoping purposes, often used for builders or constructors.
    fn new(age: usize) -> Self {
        MyStruct { age: age }
    }

    // define a method attached to a read only reference
    fn print_age(&self) {
        println!("age: {}", self.age);
    }

    // define a method attached to a mutable reference (exclusive ref)
    fn update_age(&mut self, age: usize) {
        self.age = age;
    }

    // define a method that will consume the instance of the struct
    fn consume(self) -> Self {
        return self;
    }
}
