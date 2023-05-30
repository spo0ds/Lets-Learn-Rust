## Lifetimes

The concept of lifetimes in Rust plays a crucial role in ensuring memory safety and is closely tied to the notion of ownership. Lifetimes determine the scope within which a reference remains valid, allowing Rust to enforce memory safety guarantees.

To appreciate the importance of lifetimes, let's explore some problems that motivate the use of lifetime specifiers and then delve into possible solutions.

Before we proceed, let's address the question of why lifetimes are necessary. To understand this, we need to grasp the concept of a dangling reference. In Rust, when references are used to point to variables or resources in memory, there is always an owner of that resource or variable. A dangling reference occurs when a reference points to a resource whose owner no longer exists. Accessing an invalid reference results in a dangling reference error, indicating that the resource has been deallocated. Dangling references are sometimes referred to as dangling pointers.

Let's begin with an example where we declare a variable that holds a reference to an integer:

```rust
fn main() {
    let i: &i32;
}
```

Next, we'll introduce a code segment enclosed in curly braces. Inside this scope, we'll declare a variable j:

```rust
fn main() {
    let i: &i32;
    {
        let j = 5;
    }
}
```

Subsequently, we'll set i as a reference to j:

```rust
fn main() {
    let i: &i32;
    {
        let j = 5;
        i = &j;
    }
}
```

Outside the code segment, we attempt to access the value of the variable i:

```rust
fn main() {
    let i: &i32;
    {
        let j = 5;
        i = &j;
    }
    println!("The value of i = {}", *i);
}
```

At this point, the compiler already raises an error, stating "borrowed value does not live long enough."

When we explored the ownership rules, we discussed the concept of scope, which refers to the code segment within which a variable is defined. In this case, the scope of variable j ends when the closing curly brace of the code segment is encountered. Since outside the code segment, i is pointing to a resource that no longer exists, the compiler rightfully complains. i becomes a dangling reference.

Lifetimes in Rust determine the scope for which a reference remains valid. The lifetime of variable j starts from the line in which it is initialized and ends when the code segment's curly braces are closed. The lifetimes of all variables ultimately end at the closing curly brace of the main function. To avoid dangling references, a referenced variable must live long enough and have a longer lifetime.

Dangling references are often encountered when dealing with functions. Let's consider an example:

```rust
fn main() {
    let i = 10;
    let square_i = square(i);
    println!("The square of i = {}", *square_i);
}

fn square(i: i32) -> &i32 {
    &(i * i)
}
```

Here, the compiler already complains, stating "missing lifetime specifier: this function's return type contains a borrowed value, but there is no value for it to be borrowed from."

To understand what went wrong, let's analyze the code. Variable i is the owner of the value. When we pass the value to the square function, it takes ownership of the value, and we attempt to return a reference to the variable i. However, once the function ends, i is no longer in scope, and its lifetime ends. Consequently, when we return to the main function, i no longer exists because it has been dropped after the function execution completes. As a result, square_i becomes a dangling reference.

The solution in this case is to use a reference instead of the actual value:

```rust
fn main() {
    let i = 10;
    let square_i = square(&i);
    println!("The square of i = {}", square_i);
}

fn square(i: &i32) -> Box<i32> {
    Box::new(*i * *i)
}
```

Before we proceed further, it's essential to highlight the Rust borrow checker. This module checks for lifetime-related issues and enforces various ownership and lifetime properties. It ensures that variables are initialized before use, prevents moving the same value multiple times, disallows moving a value while it's borrowed, restricts access to mutable references, and prevents mutation of a place while it's immutably borrowed.

So far, we have discussed the issue of dangling references. Another issue related to lifetimes is undetermined lifetimes.

The Rust compiler analyzes lifetime-related issues during compilation. However, there are cases where the lifetime of a variable cannot be determined at compile time.

```rust
fn main() {
    let x = 5;
    let y = 10;
}

fn greater(i: i32, j: i32) -> &i32 {
    if i > j {
        &i
    } else {
        &j
    }
}
```

In this example, we don't have dangling references since the function receives values by reference, maintaining the ownership of the variables in the main function. Therefore, anything pointing to the values of these variables will be valid references.

However, the Rust compiler still complains, stating "missing lifetime specifier." The reason for this is that Rust is uncertain about which lifetime the returning reference corresponds to. The actual reference cannot be determined at compile time and depends on the condition inside the if statement. This scenario is known as an undetermined lifetime case.

Let's consider another example to better understand this case:

```rust
fn main() {
    let s1 = "Hello";
    let v: &str;
    {
        let s2 = String::from("World");
        v = some_fn(s1, s2.as_str());
    }
    println!("The value of v is {}", v);
}

fn some_fn(first_str: &str, second_str: &str) -> &str {
    first_str
}
```

In this code, we don't have dangling pointers. Variable v points to the s1 variable, which has a scope that spans the entire main function. The error we encounter is "missing lifetime specifier." Furthermore, the error message states, "this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from first_str or second_str."

This issue highlights undetermined lifetimes. The compiler needs explicit information to determine which variable the returning reference should correspond to, as it is unable to deduce this on its own.

## Lifetimes specifier

In the previous code examples, there was an issue related to missing lifetime specifiers. To address this problem, Rust allows us to explicitly specify lifetimes using generic lifetime parameters. These parameters impose lifetime constraints on references and return values in functions.

To resolve the issue, let's modify the existing examples by using generic lifetime parameters:

```rust
fn some_fn<'a, 'b>(first_str: &'a str, second_str: &'b str) -> &'a str {
    first_str
}
```

In this code, 'a and 'b are generic lifetime specifiers that will be used by the inputs and outputs of the function. We specify that the variable first_str has a lifetime specified by 'a, and second_str has a lifetime specified by 'b. For the output, we mention that the return variable will be using the lifetime of first_str ('a). This tells the compiler that the lifetime of the returning variable is at least equal to the lifetime of first_str.

The code will now compile without any errors.

However, if we change the returning value from first_str to that of second_str, the compiler will complain again. It will raise an error of lifetime mismatch because the returning reference doesn't match the expected returning lifetime.

Generic lifetime parameters are only needed when we use references as outputs from a function.

Let's consider another example comparing two numbers with a slight change:

```rust
fn main() {
    let x = 5;
    let y = 10;
    println!("The greater number is {}", greater(&x, &y));
}

fn greater(a: &i32, b: &i32) -> i32 {
    if a > b {
        *a
    } else {
        *b
    }
}
```

In this case, Rust will not complain because the return type is not a reference.

Lifetime parameters are only specified with references. Let me modify the same program:

```rust
fn main() {
    let x = 5;
    let y = 10;
    println!("The greater number is {}", greater(&x, y));
}

fn greater<'a>(a: &'a i32, b: i32) -> &'a i32 {
    a
}
```

If I try to add a generic lifetime parameter to the variable b in the function, the compiler will complain. Therefore, generic lifetime parameters are only used with references.

**Issues with Multiple Lifetimes**

Let's revisit the program containing the greater function:

```rust
fn main() {
    let x = 5;
    let y = 10;
    println!("The greater number is {}", greater(&x, &y));
}

fn greater<'a, 'b>(a: &'a i32, b: &'b i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

In this code, there are different lifetimes for the returning variables, which can either be 'a or 'b. Since the compiler anticipates the possibility of the variable b being the returning reference, it sees in advance that the variable b does not match the required returning lifetime.

This means we need to closely consider the returning variables and take care of their lifetimes.

**Correspondence of Lifetime with Minimal Lifetime**

Let's make some changes to the same function:

```rust
fn main() {
    let x = 5;
    {
        let y = 10;
        let result = greater(&x, &y);
        println!("The greater number is {}", result);
    }
}

fn greater<'a, 'b>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

The two references passed to the greater function have different lifetimes or scopes. Variable x has a different lifetime than variable y. Since y has a smaller lifetime, it becomes a concrete lifetime. This means that all the parameters that have 'a written next to them have a lifetime at least as long as y. It also means that the variable result, which stores the returned reference, should be valid as long as y is valid. If we use the variable result outside the scope of y, we'll get a compilation error.

**Generic Lifetime Parameters with Structures**

Let's consider a scenario where we have a structure that has a field which is a reference itself:

```rust
struct Person {
    name: &str,
    age: u32,
}
```

The name field in this case is a reference to a string slice. The Rust compiler complains that it's missing a lifetime specifier in the name field. Essentially, Rust warns that the reference to the field should live as long as the struct itself. To specify this, we need to use the Rust lifetime specifier:

```rust
struct Person<'a> {
    name: &'a str,
    age: u32,
}
```

Let's now use the struct inside a program:

```rust
fn main() {
    let first_name = "John";
    let mut p1 = Person {
        name: &first_name,
        age: 40,
    };
    {
        let last_name = String::from("Shelby");
        p1.name = &last_name;
    }
    println!("{:?} is {:?} years old.", p1.name, p1.age);
}
```

The compiler will give us an error message saying that the variable last_name didn't live long enough. This is because of the requirement that the field should live as long as the structure itself.

**Reference to the Same Variable**

```rust
fn main() {
    let num = vec![1, 4, 7, 5, 9, 3];
    let return_vec = use_vec(&num, &num);
}

fn use_vec(vec1: &[i32], vec2: &[i32]) -> &[i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
}
```

In this case, I'm passing the same vector twice. As expected, without the lifetime specifier, the compiler will complain. Although we're passing the same vector, the compiler is still unable to determine which variable the output reference corresponds to, even though we know they are the same. Therefore, it needs explicit lifetimes to avoid possible confusion. Let's add the lifetime parameters:

```rust
fn use_vec<'a>(vec1: &'a [i32], vec2: &'a [i32]) -> &'a [i32] {
    if 3 > 5 {
        vec1
    } else {
        vec2
    }
}
```

## Closures : Anonymous Functions

In Rust, closures are a powerful feature that allows us to create anonymous functions. When combined with functions that take closures as arguments, they enable us to achieve remarkable functionality. Closures are defined using the || and enclosed inside {} syntax. Let's explore closures with a simple example:

```rust
fn main() {
    let x = 5;
    let square = || println!("The square of variable x is {}", x * x);
    square();
}
```

One key aspect of closures is that they capture their environment. This means that closures can access variables from the code segment in which they are defined. In the example above, the variable x is known inside the closure body. This behavior is different from functions, where only variables defined within the body or passed as arguments are accessible. In this case, even though we didn't pass the variable x to the closure, it is still accessible inside the closure body.

We can also define closures that accept input:

```rust
fn main() {
    let x = 5;
    let square = |num: i32| println!("The square of {} is {}", num, num * num);
    square(x);
}
```

We can reuse closures just like functions in subsequent code:

```rust
fn main() {
    let x = 5;
    let square = |num: i32| println!("The square of {} is {}", num, num * num);
    square(x);

    let y = 10;
    square(y);
}
```

This means that closures are reusable and can be called multiple times.

Now let's explore some interesting aspects of closures.

If we redefine a variable with a different closure, the new definition will be used:

```rust
fn main() {
    let x = 5;
    let square = |num: i32| println!("The square of {} is {}", num, num * num);
    let square = |num: i32| println!("The cube of {} is {}", num, num * num * num);
    square(x);

    let y = 10;
    square(y);
}
```

Ownership rules for closures are applied similarly to functions:

```rust
fn main() {
    let user_info = |general_info: String, name: &str, age: u32| {
        println!("{} \t {} \t {}", general_info, name, age)
    };
    let general_info = String::from("The details are");
    let (user_name, user_age) = (String::from("James Shelby"), 29);

    user_info(general_info, &user_name, user_age);
}
```

In this case, the ownership of the general_info variable is transferred to the variable inside the closure, while the ownership of user_name remains with user_name.

Closures are capable of inferring their input and output types. Unlike functions where we have to explicitly specify the types, closures can deduce them:

```rust
fn main() {
    let square = |num| num * num;
    let x = 5;
    square(x);
}
```

If we call the closure with a different type of input, Rust will complain because it automatically infers the type of num based on the type of x and determines the return type.

Furthermore, closures can be passed as parameters to functions:

```rust
fn main() {
    let division_status = |y: f32| {
        if y != 0.0 {
            true
        } else {
            false
        }
    };

    division(10.0, 5.0, division_status);
}

fn division<F: Fn(f32) -> bool>(x: f32, y: f32, f: F) {
    if f(y) {
        println!("The division result is {}", x / y);
    } else {
        println!("Division is not possible");
    }
}
```

In this example, we define a closure division_status that checks if the division is possible by checking if y is not equal to 0.0. We then pass this closure as a parameter to the division function, which takes a closure that accepts a f32 and returns a bool. Inside the function, we use the closure to determine whether the division is possible and perform the division accordingly.
