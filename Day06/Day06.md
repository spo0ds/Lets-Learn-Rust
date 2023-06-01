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

We will now focus on the details of modules in Rust.

To create a new crate inside the source folder, we can create a file named "crate1.rs". Within this file, we can define modules by using the mod keyword. For example:

```rust
mod maths {}
```

In this case, we are creating a module named "maths" to handle math-related operations.

Modules can contain other modules as well. To create a submodule inside the "maths" module, we can do the following:

```rust
mod maths {
    mod basic_math {}
}
```

This means that we now have an upper-level module called "maths" which contains a submodule called "basic_math".

Inside the submodule, we can define functions for specific computations. For example, we can define a function for multiplying two numbers:

```rust
mod maths {
    mod basic_math {
        fn mul(x: &i32, y: &i32) -> i32 {
            x * y
        }
    }
}
```

Now, the function defined inside the module can be used by other functions within the same crate. For instance, we can use the multiplication function to compute the area of a rectangle:

```rust
mod maths {
    mod basic_math {
        fn mul(x: &i32, y: &i32) -> i32 {
            x * y
        }
    }
}

fn rect_area(length: &i32, breadth: &i32) -> i32 {
    maths::basic_math::mul(length, breadth)
}
```

However, we encounter an error message stating that the module "basic_math" is private. The parent module cannot see the functions within the child module, but the child module can see the functions within the parent module. To make sure that functions are accessible within the child module, we need to use the pub keyword for both the child module and the function itself.

Now, let's use the crate "crate1.rs" inside the main function. Note that the crate name is also considered a module from the perspective of the outside world. At the top level of the crate, we have "crate1" as the module. Inside this module, we have the "maths" module, and inside the "maths" module, we have the submodule "basic_math". Inside "basic_math", we have the "mul" function.

Since "rect_area" is not included in any module, it will be included in the default module, which is the crate itself. This means that anything not included in any module within this crate will be included in the default module, which is the name of the crate itself.

In order to access items at deeper levels, we need to declare them as pub.

```rust
mod crate1;

fn main() {
    let rect1 = Rectangle {
        length: 10,
        breadth: 7,
    };
    println!(
        "The area of the rectangle is {}",
        crate1::rect_area(&rect1.length, &rect1.breadth)
    );
}

struct Rectangle {
    length: i32,
    breadth: i32,
}
```

However, the compiler is still complaining that "rect_area" is private. To resolve this, we need to make the "mul" function public by adding the pub keyword in front of it.

Inside the "basic_math" module, let's add a function for printing:

```rust
mod maths {
    pub mod basic_math {
        pub fn mul(x: &i32, y: &i32) -> i32 {
            x * y
        }

        fn printing(num: &i32) {
            println!("The result is {}", num);
        }
    }
}
```

We can now see that we haven't used the pub keyword with the "printing" function, and the compiler is happy. Functions within a module can access each other without the pub keyword.

Now, let's see how a child module can access functions defined in a parent module. We define a function inside the main module, "crate1":

```rust
fn parent_fn() {
    println!("Here from the parent module");
}
```

To access this function inside the "printing" function, we can use the fully qualified path:

```rust
fn printing(num: &i32) {
    println!("The result is {}", num);
    crate::parent_fn();
}
```

Although the function "parent_fn" is not public, we can still access it without any issues.

Sometimes, it may not be convenient to always mention the full path of a certain function, especially if it is used repeatedly. In this case, we can use the use statement to reduce the path information. For example, in the "rect_area" function, we can write:

```rust
pub fn rect_area(length: &i32, breadth: &i32) -> i32 {
    use maths::basic_math;
    basic_math::mul(length, breadth)
}
```

We can also add more specific information with the use statement:

```rust
pub fn rect_area(length: &i32, breadth: &i32) -> i32 {
    use maths::basic_math::mul;
    mul(length, breadth)
}
```

Sometimes, we may want to include all modules and functions within the same crate, which is the main crate. Typically, we would separate them into different files, but for some reason, let's assume that we want to use a single file as the main crate. To achieve this, we need to make a few changes.

First, copy all the code from "crate1.rs" and paste it into "main.rs". Since all the statements are in the same file now, we can remove the line:

```rust
mod crate1;
```

Since "rect_area" is in the same file and not inside any module, we can remove "crate1" from its beginning:

```rust
crate1::rect_area(&rect1.length, &rect1.breadth) // remove this

rect_area(&rect1.length, &rect1.breadth)
```

This also means that we don't need to make it public anymore, so we can remove the pub keyword from its beginning.

We can notice that we have an error in the "printing" function. This is because inside modules, especially the child module, we need to explicitly tell them where to look for the function. So we'll remove "crate1" while calling the "parent_fn" function:

```rust
fn printing(num: &i32) {
    println!("The result is {}", num);
    crate::parent_fn();
}
```

Now let's cover a few important points regarding modules.

The first point is the privacy of structures used inside modules. To explain this, let's create a new crate by creating a new file named "crate2.rs". Inside this file, we'll create a module called "Person":

```rust
mod Person {
    struct Personal_info {
        age: u8,
        education: String,
    }
}
```

Next, we'll create an implementation block for the structure:

```rust
impl Personal_info {
    pub fn new(new_edu: &str) -> Self {
        Self {
            education: String::from(new_edu),
            age: 20,
        }
    }
}
```

Now, outside the module body, we'll create a function that we'll use in the main crate, so it needs to be public:

```rust
pub fn person() {
    println!("From the person function");
}
```

Moving to "main.rs", we'll use the function:

```rust
mod crate2;

fn main() {
    crate2::person();
}
```

In the "person" function, we'll add some code:

```rust
pub fn person() {
    println!("From the person function");
    let mut p1 = Person::Personal_info::new("Bachelor's in Computer Science");
}
```

The compiler is already complaining that the struct is private. Let's make the struct public by writing pub in front of it. Now the compiler is happy.

Let's update the value of the variable p1:

```rust
p1.education = String::from("Master's in Computer Science");
```

The compiler is not happy, saying that it's a private field. This leads to an important point: By default, fields within a structure are private, even if the structure itself is public. We'll write pub in front of the "education" field, and the compiler is happy.

Now, let's create another instance of the struct:

```rust
let p2 = Person::Personal_info {
    age: 25,
    education: String::from("Bachelor's in Accounting"),
};
```

This gives us an error because we can't create the Personal_info struct directly. It contains a private field, which is the "age". In the previous case, when using the "new" function, we were able to do so because the "new" function is defined inside the module where we have defined the struct. Within the module at the same level, elements can see each other. However, in this case, since the function "person" is outside the module, it is not allowed to see the items inside the module. This means we need to make the "age" public as well if we want to update it outside the module.

So, let's add pub in front of the "age" field as well. Now the code will compile successfully.

Let's create another crate called "crate3.rs". Inside this file, we'll create a module called "travel_history":

```rust
mod travel_history {
    pub enum Conveyance {
        Car,
        Train,
        Aeroplane,
    }
}
```

Next, we'll use the enum types by creating a function called "allowance" outside the module:

```rust
pub fn allowance() {
    let t1 = travel_history::Conveyance::Car;
    let t2 = travel_history::Conveyance::Train;
}
```

Moving to "main.rs", we'll call the function:

```rust
mod crate3;

fn main() {
    crate3::allowance();
}
```

In "crate3.rs", we can see that the compiler is complaining about the privacy of "Conveyance". Let's add the pub keyword in front of the enum. Now you may note that there are no more issues. When we make an enum public, we don't need to specify its variants to be public. They will become public automatically.

## Utilizing External Crates

In the realm of programming, it is advantageous to leverage existing code created by others, as this eliminates the need to reinvent the wheel. By incorporating external code into your own projects, you can benefit from the functionality and features provided by those code libraries, much like using modules defined within separate crates.

When seeking out pre-existing code from other developers, a valuable resource to explore is the official website called crates.io. This platform serves as a repository where individuals contribute their code for others to utilize.

To incorporate a specific crate, such as the "array_tool" crate available on crates.io, you will need to modify your cargo.toml file. Within the cargo.toml file, add the following lines under the [dependencies] section:

```toml
[dependencies]
array_tool = "1.0.3"
```

Having included the desired crate in your project, you can now proceed to utilize its functionality in your code. For instance, let's consider the scenario where we want to use the vec function from the "array_tool" crate.

In your main.rs file, import the vec function using the use statement:

```rust
use array_tool::vec::*;

fn main() {}
```

By importing the vec function in this manner, it becomes accessible within the scope of your code.

To demonstrate the practical use of the imported function, let's declare a couple of vectors and find their intersection:

```rust
use array_tool::vec::*;

fn main() {
    let x = vec![1, 2, 3, 7, 6, 5];
    let y = vec![1, 10, 6, 8, 11, 15];

    let intersection = x.intersect(y);
    println!("The intersection of the two vectors is: {:?}", intersection);
}
```

In the provided code snippet, we define two vectors, x and y, containing different elements. By utilizing the imported vec function, we can find the intersection of these two vectors. The resulting intersection is then printed to the console using the println! macro.
