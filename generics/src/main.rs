use core::fmt::{Debug, Display};

fn main() {
    let with_string = MyGenericType {
        inner: String::from("abc"),
    };

    with_string.do_something();
    let cloned = with_string.clone_inner();

    println!("cloned: {cloned}");

    let with_int = MyGenericType { inner: 10 };
    with_int.do_something();
    let cloned = with_int.clone_inner();

    println!("cloned: {cloned}");

    generic_fn(&with_string);
    generic_fn(&with_int);

    println!("result from add_42_millions(25)={}", add_42_millions(25));

    println!("formatted: {}", format_name("abc"));
}

// create a Generic type, and
// uses it as its attribute.
struct MyGenericType<T> {
    inner: T,
}

// declare a generic method.
// if the type is generic over T, the impl also needs to be generic over T
impl<T> MyGenericType<T>
// enforce some trait bounds (constraints) on the generic type T
where
    T: Debug,
{
    fn do_something(&self) {
        println!("doing something with inner: {:?}", self.inner);
    }
}

impl<T> CloneInner for MyGenericType<T>
where
    T: Clone,
{
    // define that we'll return a usize
    type Result = T;
    fn clone_inner(&self) -> Self::Result {
        // return inner
        return self.inner.clone();
    }
}

trait CloneInner {
    // define that the trait has an associated type
    // that the implementation needs to declare
    type Result;
    // use the associated type as the return type of a method
    fn clone_inner(&self) -> Self::Result;
}

fn generic_fn<T>(a: &T)
where
    T: CloneInner,
    T::Result: Debug,
{
    let cloned = a.clone_inner();
    println!("generic_fn: cloned value={:?}", cloned);
}

// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

// a function can be generic by accepting a generic argument,
// but it can also return a generic type that satisfies a trait.
// The caller will not know the specific typed and can only use
// the methods on the trait. Useful for hiding underlying data.
fn format_name(name: impl Display) -> impl Display {
    format!("Hello {name}")
}
