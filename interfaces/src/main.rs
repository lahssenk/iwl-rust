fn main() {
    let john = Person {
        name: "john".to_owned(),
        age: 20,
    };
    john.print_age();
    john.describe();

    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog {
            name: String::from("Fido"),
        }),
    ];
    for pet in pets {
        println!("Hello {}!", pet.name());
    }
}

trait Pet {
    fn name(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat;

impl Pet for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("The cat") // No name, cats won't respond to it anyway.
    }
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
