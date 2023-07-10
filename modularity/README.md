# Modularity

This chapter presents how to modularize your code in rust.

A build unit in rust is called a crate, consider it being a package.
Then a crate can be composed/divided into modules.

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
-> if you have many files in the same folder, create a mod.rs file and add the list of internal modules with `mod <moduleName>;` statements;

# Visibility

By default, a symbol (type, func, method, const) is private, which makes
it accessible only to child modules.

To make a symbol public, prefix its name with the `pub` keyword.