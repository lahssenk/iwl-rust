# Pointers

This chapter covers pointers, references lifetimes and the ownership model of rust.

## References

Rust presents an abstraction around pointers, called references. A reference provides
the guarantee to not be nil at compile time, making it safer to share access to resources
via pointers.

Types of references:
- immutable, using the `&` sign in front of a variable name
- mutable, using the `&mut` keyword in front of a variable name

The rust compiler ensures that `&mut` is exclusive (no other `&mut` or `&` can exist)
and that there can be many `&` safely. (allowing for multiple concurrent readers or a unique writer)

# Send and Sync traits

Each basic type is defined in the language with some Send and Sync capability.
A type is Send if the value can be sent from a thread to another safely.
A type is Sync if a reference to the value can be sent from a thread to another safely.

# Smart Pointers

In rust, raw references are very powerful and form the foundation for its "fearless concurrency" paradigm. Despite being a power concept, some types dedicated to sharing data are included in the language to help with more complex use-cases.

- Box: a generic pointer to some value that will be stored in the heap. Useful because every value needs to have a knows size at compile time, growable types therefor need to be stored on heap so that a static variable (header type) can be used as a handle to the underlying backing data
- Rc (Reference Counter): useful for keeping track of references to a value when it is not known in advance which will be the last reference to stay alive and when to drop the value (free memory)
- Arc (Atomic Reference Counter): slower but thread safe reference counter. It makes a type Send
- Mutex: a wrapper around a type that provides locking primitives. It returns a lock guard around the value, that automatically unlocks the mutex when the guard goes out of scope (dropped)