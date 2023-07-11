fn main() {
    let john = Person {
        name: "john".to_owned(),
        age: 20,
    };
    john.print_age();
    john.describe();

    // the size in memory of the vector needs to be knows in advance.
    // We want the vec to hold values that implement the trait, but
    // they can be of homogeneous types. We use trait objects (syntax `dyn <trait>`)
    // for this.
    // The vector is a header type on the Stack, with ptr, capacity and len.
    // The pointers holds the address of the data on the Heap.
    // Each item is stored on the heap with Box.
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat),
        Box::new(Dog {
            name: String::from("Fido"),
        }),
    ];
    for pet in pets {
        println!("Hello {}!", pet.name());
    }

    println!(
        "{} {}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    println!("{}", std::mem::size_of::<&dyn Pet>());
    println!("{}", std::mem::size_of::<Box<dyn Pet>>());

    // use derived traits
    let p1 = Player::default();
    let p2 = p1.clone();
    println!(
        "Is {:?}\nequal to {:?}?\nThe answer is {}!",
        &p1,
        &p2,
        if p1 == p2 { "yes" } else { "no" }
    );
}

// trait objects --------
trait Pet {
    fn name(&self) -> String;
    fn method_with_default_impl(&self) {
        println!("the default impl");
    }
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

// deriving traits
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}
