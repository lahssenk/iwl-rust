# Interfaces

This chapter presents the concepts in rust that help build code that
relies on behavior based abstractions instead of specific implementations.

An interface is a type defining a set of methods, so that a receiver func/type/method
can rely on the parameters it is provided, to implement such defined functionaity.

## Traits

In rust, the common concept of an interface is defined as a Trait.
A trait is a set of methods that must be implemented by a type.
In rust, implementation of a trait is explicit, implementing the methods
is not enough, a type implements a trait by name. This is useful because
sometimes multiple independant modules/crates can define traits with similar
names, with potentially the same set of methods/signatures.

Syntax:
```
trait PrintAge{
    fn print_age(self)
}
```