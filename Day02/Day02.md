## Ownership Rules, Primitive and Non-Primitive Types

Ownership is a fundamental concept in Rust that ensures memory safety guarantees. Understanding ownership is crucial for proficiency in Rust, as it governs how values are managed in the language. Rust's ownership rules are simple and can be summarized in three key rules:

- Each value in Rust has a variable that is its owner.
- There can only be one owner of a value at a time.
- When the owner goes out of scope, the value will be dropped.

To grasp these rules, let's familiarize ourselves with some basic terminologies in Rust.

In Rust, the terms "copy" and "move" have specific meanings:

```rust
fn main() {
    let mut x = 10;
    let mut y = x;
    println!("The value of x = {} and the value of y = {}.", x, y);
}
```

Output: The value of x = 10 and the value of y = 10.

In this example, when y = x is executed, Rust makes a copy of the value stored in x. This means that x and y have separate memory locations, each storing the value 10. This behavior is possible because integers in Rust are primitives, which have a fixed size and can be easily copied.

Now, let's consider a similar program that involves string variables:

```rust
fn main() {
    let mut a = String::from("Hello");
    let mut b = a;
    println!("The value of x = {} and the value of y = {}.", a, b);
}
```

Error: borrow of moved value: a

In this case, Rust's ownership concept comes into play. When a = String::from("Hello") is executed, it assigns a memory location to store the value "Hello", and a becomes the owner of that value.

When the value of a is then assigned to b using let mut b = a, the ownership of the value stored in memory is moved from a to b. In other words, a move operation occurs, changing the ownership.

Now, since the ownership has been transferred, a is no longer a valid variable. This behavior is in accordance with the first ownership rule. If we try to use the variable a after the ownership transfer, we get the corresponding error message.

Rust categorizes variables into two types: primitives and non-primitives. These types refer to the variable's behavior, not its data type. Primitives have a fixed size and cannot grow, while non-primitives can dynamically grow and be empty.

Primitives include variables of types such as integers, floats, booleans, characters, and arrays. Non-primitives include vectors and strings.

With primitives, Rust simply makes a copy when an assignment operation occurs. However, with non-primitives, Rust performs a move operation that changes the ownership when an assignment occurs. As a result, you need to consider and manage ownership only for non-primitive types.

Let's correct our code example so that it executes correctly:

```rust
let mut b = &a;
```

he &a syntax allows us to create a reference that refers to the value of a without owning it. This means that the ownership remains with the variable a, while variable b can be used as a reference to access or point to the value of a. This satisfies the Rust ownership rule, ensuring that each value has a single owner. The compiler is happy with this code.

When a variable is used as a reference to another variable, we say that the referencing variable has "borrowed" the value by reference.

Borrowing an item from a friend doesn't change the ownership of the item. Similarly, referencing a variable in Rust doesn't change its ownership.

```rust
fn main() {
    let num = vec![1, 2, 3];
    let num2 = &num;
    println!("Num: {:?}, num2: {:?}", num, num2);
}
```

What if we genuinely want to make a copy of the variable and retain ownership?

To achieve this, Rust provides a function called clone:

```rust
fn main() {
    let num = vec![1, 2, 3];
    let num2 = num.clone();
    println!("Num: {:?}, num2: {:?}", num, num2);
}
```

Now, let's delve into the third ownership rule.

To understand this rule, we need to grasp the concept of scope. In Rust, a scope is defined using curly braces {}. When Rust finishes executing a scope, all data values within that scope are discarded or dropped, and the memory they occupied is freed for other uses.

```rust
{
    let name = String::from("James");
}
println!("Name: {}", name);
```

The compiler will complain because of the ownership rule violation. Once outside the scope, the variable name no longer exists. It is considered out of scope.
