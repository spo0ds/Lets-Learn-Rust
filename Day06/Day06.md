## Modules: Organizing and Encapsulating Code in Rust

As our codebase grows, it becomes essential to organize and encapsulate our code. We don't want every part of our program to have knowledge about the internal workings, functions, and variables of a particular chunk of code, such as authentication. Instead, we may prefer to expose a simplified method, like "login," while keeping the rest of the code hidden. To address these needs, Rust provides a module system.

Let's start with the concept of a package. So far, we've been writing all our code in a single file within the default module. However, as our project becomes more complex, we need a way to structure our code. Modules help us achieve this organization. Traits, which contain modules, play a significant role. Traits can be of two main types: binary traits and library traits. A binary trait represents code that can be executed independently, while a library trait is designed to be reused by other programs. Crates, on the other hand, are families of modules that allow us to organize our code effectively.

Let's create a simple package. We step outside the src folder and use the command cargo new <package_name> to create a new package. The newly created package will have a folder named <package_name>, like "my_package" in my case, and within that folder, there will be an src folder containing main.rs. main.rs is the binary crate that we can execute. Rust conventionally creates a binary crate with the same name as the package when main.rs is defined in the src directory. Additionally, if a package has a main.rs file, it becomes the crate root. This means that it will be the source file from which the Rust compiler starts building the code. Every subsequent crate will be used and referenced from the main module. A library crate can be referenced from a binary crate. Libraries do not have an entry point, so they cannot be run directly. They are meant for sharing functionality. Multiple library crates are not allowed, and a library crate is typically created with the name lib.rs.

Let's create a library crate inside the my_package/src directory:

```rust
fn printing() {
    println!("From inside the library file");
}
```

Now, in the binary crate main.rs, we can use the function defined in lib.rs:

```rust
use my_package;
fn main() {
    my_package::printing();
}
```

You may notice that the Rust compiler is not happy with this and says "The function is private." To make the function accessible to the outside world, we can make it public:

```rust
pub fn printing() {
    println!("From inside the lib.rs file");
}
```

Now we can run the code. The idea is that lib.rs is meant for sharing functionality.

We can create additional crates by adding new files. Let's create two more crates. Inside the src folder, we'll create a new file called crate1.rs and another file called crate2.rs. Inside these crates, we'll write similar code as in lib.rs:

```rust
pub fn printing() {
    println!("From inside the crate2 file");
}
```

Inside the library crate, we include these two crates:

```rust
pub mod crate1;
pub mod crate2;
```

The mod keyword is used to represent a Rust module. Modules are hierarchically lower than crates, and a crate may contain more than one module. By default, every crate has a single module, which is the crate itself.

Now let's use these two crates inside the main function:

```rust
use my_project::crate1;
use my_project::crate2;

fn main() {
    crate1::printing();
    crate2::printing();
}
```
