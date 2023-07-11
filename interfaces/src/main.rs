fn main() {
    let john = Person {
        name: "john".to_owned(),
        age: 20,
    };
    john.print_age();
    john.describe();
}

trait PrintAge {
    fn print_age(self);
}

struct Person {
    name: String,
    age: u8,
}

impl PrintAge for &Person {
    fn print_age(self) {
        println!("age: {}", self.age);
    }
}

impl Describe for &Person {
    fn describe(self) {
        println!("name={}, age={}", self.name, self.age);
    }
}

// trait Describe requires that trait PrintAge is also satisfied.
trait Describe: PrintAge {
    fn describe(self);
}
