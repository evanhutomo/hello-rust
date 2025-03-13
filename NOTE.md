Traits in rust are similar to interfaces in other languages. They define a set of methods that a type must implement in order to be considered a member of that trait.

There are 3 type of traits in rust:
1. **Default Traits**: These are traits that are implemented by default for all types. For example, the `ToString` trait is implemented for all types that implement the `Display` trait.
2. **Custom Traits**: These are traits that are defined by the user. For example, the `Animal` trait is a custom trait that defines a set of methods that an animal must implement.
3. **External Traits**: These are traits that are defined in external crates. For example, the `std::fmt::Debug` trait is defined in the standard library and is used to format a type for debugging purposes.
