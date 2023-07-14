# Modularity

This chapter presents how to modularize your code in rust.

A build unit in rust is called a crate, consider it being a package.
Then a crate can be composed/divided into modules.

# Resources
- [separating modules into multiple files](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html)
- [referring to item in module tree](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)

# Modules

A module can be declared many ways:
- embed a submodule in the same file, by defining a named `mod` block
```
mod something {
    fn do_something(){println!("hello");}
}
```
- extract the module into another file
-> the name of the module is the name of the file, example something.rs is implicitly declared as `mod something`
-> if you want to group modules together in a parent module (example named `sub`), the old way was to create `sub/mod.rs`, add `mod` statements to make inner modules visible, and then mount module `sub` into scope in main with `mod sub`. The new way is to replace `sub/mod.rs` with `sub.rs`

# Visibility

By default, a symbol (type, func, method, const) is private, which makes
it accessible only to child modules.

To make a symbol public, prefix its name with the `pub` keyword.