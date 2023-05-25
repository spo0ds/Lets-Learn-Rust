## Installation

To install Rust, you can follow these steps:

- Visit the [official Rust page](https://www.rust-lang.org/tools/install) that provides installation instructions tailored to your specific environment.
- Once on the official page, follow the provided instructions to install Rust on your system. These instructions will guide you through the installation process, ensuring that you have the necessary dependencies installed.
- If you plan to use Visual Studio Code as your editor, you should also install the Rust support extension called "rust-analyzer" to enhance your Rust development experience.

After successfully installing Rust, you can verify the installation by running the command rustc --version in a terminal. This command will display the version of the Rust compiler installed on your system. Rust also comes with its own package manager, similar to npm, pip, or yarn, which allows you to manage dependencies for your projects.

## Running a Simple Rust Program

In Rust, source code files must have the extension .rs. Let's create a simple "Hello, World!" program to demonstrate running Rust code. Create a new file with the .rs extension, such as helloWorld.rs, in your preferred text editor or IDE, and add the following code:

```rust
fn main() {
    println!("Hello, World!");
}
```

To compile the code, open a terminal or command prompt and navigate to the directory where helloWorld.rs is saved. Run the command

`rustc helloWorld.rs`

If the compilation is successful, you will see an executable file named helloWorld or helloWorld.exe, depending on your operating system.

To execute the compiled code, run the command

`./helloWorld`

This will execute the compiled program, and you should see the output Hello, World! printed in the console.

The above steps are suitable when working with a simplified program consisting of a single file. However, in real-world scenarios, programs tend to become more complex and may include multiple files. To manage such projects, it is recommended to use the cargo utility, which provides enhanced project management and build capabilities.

## Managing Projects with Cargo

To initialize a new Rust project using Cargo, you can follow these steps:

- Delete the helloWorld.rs file and its compiled output if you have previously compiled it.

- Open a terminal or command prompt and navigate to the directory where you want to create your Rust project.

- Run the command `cargo init`. This command initializes a new Rust project in the current directory, creating the necessary files and directories.

After running cargo init, you will see a Cargo.toml file, which is similar to a package.json file in Node.js. It contains information about the project, such as its name and version, and can also include dependencies.

Inside the project directory, you will find a src folder that contains the source files with the .rs extension. By default, cargo init creates a main.rs file inside the src folder, which serves as the entry point for your application.

To compile and run your project using Cargo, run the command `cargo run` in the terminal or command prompt from the project directory. Cargo will handle the compilation and execution process for you.

The compiled code will be stored in the target/debug directory relative to your project. You can also directly execute the compiled code by running ./target/debug/<project_name> in the terminal or command prompt, where <project_name> is the name of your project.

By utilizing Cargo, you benefit from its powerful features, such as dependency management, building, testing, and more, which simplify the development and distribution of Rust projects.

## Basic Programming

If you are already familiar with programming in other languages, getting started with Rust should be relatively straightforward. In this section, we'll cover some basic concepts to help you write programs in Rust.

## Comments and Print Commands

In Rust, you can add comments using // for single-line comments or /\* \*/ for multi-line comments.

```rust
// This is a simple "Hello, World!" program written in Rust.

/* simple hello world program
    written in rust
*/

fn main() {
    println!("Hello, world!");
}
```

The println! macro is used to print output in Rust. The exclamation mark (!) denotes that println! is a macro and not a regular function call. It allows you to format and print text to the console.

To output a constant number using println!, you need to include a placeholder inside the double quotes, which will be replaced with the constant:

```rust
println!("{}", 10);
```

You can also use placeholders to print string values:

```rust
fn main() {
    println!("I'm learning Rust from {}", "Spo0ds Github");
}
```

Output : I'm learning Rust from Spo0ds Github

The println! macro will print the formatted output, and any subsequent print statements will be displayed on the next line. To print output on the same line, you can use the print! macro:

```rust
fn main() {
    print!("I'm learning Rust from {}.", "Spo0ds Github");
    print!(" It's an awesome walkthrough.");
}
```

Output : I'm learning Rust from Spo0ds Github. It's an awesome walkthrough

The print! macro keeps the text as is and does not add a newline character.

```rust
fn main() {
    print!("I'm learning Rust from {}.", "Spo0ds Github");
    print!(" It's an awesome               walkthrough.");
}
```

**Use of backlash**

Backslashes (\) are used to escape characters in Rust. For example, you can use backslashes to include quotes inside the println! macro:

```rust
fn main() {
    println!("\nJames said, \"This repo is what I needed as it covers everything from basics.\"");
}
```

In this example, the \n represents a newline character, and the \" represents a double quote character.

Output : James said, "This repo is what I needed as it covers everything from basic."

**Positional arguments**

You can also use positional arguments or named arguments in println! to control the order of the printed values:

```rust
fn main() {
    println!("I'm learning {1} from {0} Github.", "Spo0d's", "positional arguments");
}
```

This will output: "I'm learning positional arguments from Spo0d's Github."

**Named Arguments**

Similarly, you can use named arguments to achieve the desired output:

```rust
fn main() {
    println!("Learning {language} is fun.", language = "Rust");
}
```

This will output: "Learning Rust is fun."

## Variable and Scalar Data Types

In Rust, variables serve as storage locations that programs can manipulate. They are identifiers used to refer to memory locations that hold values during the execution of a program. The data type associated with a variable determines its size and layout. Let's explore variables in Rust in more detail.

To create a simple variable in Rust, you can use the let keyword followed by the variable name and its type annotation:

```rust
fn main() {
    let num: i32 = 7;
}
```

When assigning a value to a variable, Rust requires additional information about the type of the variable. This information is specified after a colon (:) using a type annotation. The compiler uses this information to infer the type from the assigned value.

For example, if the value assigned to the variable num is changed to 7.7, the type information would be inferred as f64:

```rust
fn main() {
    let num: f64 = 7.7;
}
```

By default, variables in Rust are immutable, meaning their values cannot be changed once assigned. If an attempt is made to assign a new value to an immutable variable, it will result in a compilation error:

```rust
fn main() {
    let num: i32 = 7;
    println!("The value of the variable num is {}.", num);

    num = 10; // Error: cannot assign twice to immutable variable
}
```

This immutability feature is one of the many additions in Rust that promotes safer coding practices. By assuming that the value of a variable will never change, the code that operates on it can be designed to work correctly. If the value changes unexpectedly, the code might not function as intended. Making variables immutable by default helps avoid such issues.

However, it is still possible to change the value of a variable by explicitly declaring it as mutable using the mut keyword:

```rust
fn main() {
    let mut num: i32 = 7;
    println!("The value of the variable num is {}.", num);

    num = 10;
    println!("The value of the variable num is {}.", num);
}
```

Output : The value of the variable num is 7.
The value of the variable num is 10.

In this case, the mut keyword allows the value of the variable num to be modified.

There are some rules to follow when naming variables in Rust:

- The name of a variable can consist of letters, digits, and underscores (\_) only.
- The variable name must begin with a letter or underscore and should not start with a digit.
- Rust is case-sensitive, so upper and lower case letters are distinct.

Now, let's discuss data types in Rust.

**Data Types**

Rust data types are divided into two groups: scalar and compound data types. Scalar data types represent a single value, while compound data types can contain multiple values.

Let's focus on scalar data types first.

**Scalar Data Types**

Scalar data types represent a single value. Rust has four primary scalar types:

- Integers
- Floating-point numbers
- Booleans
- Characters

These types are recognizable in other programming languages as well.

**Integers**

Integers are numbers without a fractional component. They can be further categorized as signed and unsigned, depending on whether the numbers can be positive or negative. In Rust, the i prefix indicates signed integer types, while the u prefix indicates unsigned integer types. For example, i8, i16, i32, and i64 represent signed integer types.

To get the maximum value of an integer data type, you can use the MAX constant from the respective type:

```rust
fn main() {
    println!("The maximum value of the i8 data type is {}.", std::i8::MAX);
}
```

Output : The maximum value of the i8 data type is 127.

Similarly, unsigned integer types have their own MAX constant, but they can only store positive values:

```rust
fn main() {
    println!("The maximum value of the u8 data type is {}.", std::u8::MAX);
}
```

Output : The maximum value of the u8 data type is 255.

**Floating-Point Numbers**

Floating-point numbers can represent numbers with decimal points. Rust has two floating-point types: f32 and f64. The f64 type is the default for floating-point variables in Rust.

Note that performing mathematical operations on variables with different data types is not allowed:

```rust
fn main() {
    println!("The sum is {}.", 3 + 4.2); // cannot add a float to an integer
}
```

**Booleans**

Booleans have two possible values: true and false. They are useful for making logical decisions in code.

```rust
fn main() {
    let mut num: i32 = 7;
    num = 10;
    let x: bool;

    if num != 10 {
        x = false;
    } else {
        x = true;
    }

    println!("Is x equal to 10? {}", x);
}
```

Output : Is x equal to 10? true

**Characters**

Characters can represent single digits, numbers, or letters. They are always enclosed in single quotes.

```rust
fn main() {
    let gpa = 'A';
    println!("I got {} in my school.", gpa);
}
```

Output : I got A in my school.

Arithmetic operations cannot be performed directly on characters since they are not of integer or float types.

To initialize a mulpitle variables:

```rust
fn main() {
    let (x,y) = (1,2.2);
    println!("The x cordinate is {} and y cordinate is {}", x,y);
}
```

Output : The x cordinate is 1 and y cordinate is 2.2

These two variables has different types.

Sometimes we may be dealing with very large numbers inside our program.To make the large number more redeable, Rust allows the uses of "\_".

```rust
fn main() {
    let large_number:i64= 1_000_000_000_000_000;
    println!("The large number is {}", large_number);
}
```

Output : The large number is 1000000000000000

**Integer Overflow**

```rust
fn main() {
    let x:u8 = 256;  // literal out of range for `u8`
}
```

The upper limit for the u8 type is 255 and I'm trying to excede its limit.

Rust also has very easy mechanism to display the decimal number in other base such as octal, binary and hexa.

**Other Base**

```rust
fn main() {
    let x:u8 = 255;
    println!("Hex Value : {:X}", x);
    println!("Octal Value : {:o}", x);
    println!("Binary Value : {:b}", x);
}
```

Output : Hex Value : FF
Octal Value : 377
Binary Value : 11111111

The Rust assumes snake case for the variable.It refers to the style of writing in which first letter is lower case.

**Operation on variable of different types**

```rust
fn main() {
    let a=14;
    let b = 7.5;
    let sum = a+b as i32;
    println!("Sum as n2 as i32 is {}", sum);
}
```

Output : Sum as n2 as i32 is 21

The data type of n2 will not be permanently changed to i32.It would just be treated as i32 with the addition command.

To get the accurate result, we should change the value of n1 as of type f64.

```rust
fn main() {
    let a=14;
    let b = 7.5;
    let sum = a as f64+b;
    println!("Sum as n1 as f64 is {}", sum);
}
```

Output : Sum as n1 as f64 is 21.5

## Shadowing and Constants

Let's explore the concepts of shadowing and constants in Rust.

**Shadowing**

Shadowing refers to the concept of updating or declaring a variable with the same name that has been previously used or declared in the program. There are several scenarios where shadowing can occur.

In the first case, we shadow a variable using the same name and the let keyword.

```rust
fn main() {
    let num = 1;
    let num = 2;
    println!("The value of num is {}.", num);
}
```

Output: The value of num is 2.

Here, the variable num is shadowed, and its previous value is replaced by the new value of 2.

In the second case, we shadow a mutable variable with an immutable one.

```rust
fn main() {
    let mut num = 1;
    let num = 2;
    println!("The value of num is {}.", num);
}
```

Output: The value of num is 2.

In this case, the variable num is initially declared as mutable (mut), but then it is shadowed with an immutable variable. Therefore, the variable is treated as immutable in this scope.

The third case occurs when we change the type of a variable.

```rust
fn main() {
    let num = 1;
    println!("The value of num is {}.", num);
    let num = 2.2;
    println!("The value of num is {}.", num);
}
```

Here, the value of num is first assigned as an integer (1), and later it is shadowed with a floating-point number (2.2). This allows us to reuse the same variable name with a different type.

Lastly, shadowing can also occur within code segments that have different scopes defined by curly brackets.

```rust
fn main() {
    let num = 1;
    println!("The value of num is {}.", num);
    {
        let num = 2.2;
        println!("The value of num inside the inner scope is {}.", num);
    }
    println!("The value of num outside the inner scope is {}.", num);
}
```

Output:
The value of num is 1.
The value of num inside the inner scope is 2.2.
The value of num outside the inner scope is 1.

In this case, we have an inner scope defined by curly brackets. The variable num is shadowed inside the inner scope, and its value is different from the value of num in the outer scope.

If we make num a mutable variable and remove the let keyword inside the inner scope, the value of num in the outer scope will be changed.

**Constants**

Constants are data values that remain the same and do not change during the execution of the program. They are declared using the const keyword and must have a type annotation. Rust naming convention for constants is to use all uppercase letters with underscores (\_) between individual words.

```rust
fn main() {
    const PI: f64 = 3.14;
    println!("The value of pi is {}.", PI);
}
```

Constants are useful when you have a value that should not be modified throughout the program's execution.

The key differences between mutable variables and constants are:

- You are not allowed to use the mut keyword with constants.
- Constants are declared using the const keyword instead of let.
- Constants must have their type explicitly annotated; the compiler will not infer it.
- Rust naming convention for constants is to use all uppercase letters with underscores (\_) between individual words.
