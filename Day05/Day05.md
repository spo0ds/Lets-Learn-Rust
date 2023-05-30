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

## In-Depth Understanding of Closure Capturing Environment

Let's delve into a detailed exploration of closure capturing environments in Rust.

**Borrowing with Immutable Reference**

When a closure uses an immutable reference to a variable inside the code, it captures the environment as follows:

```rust
fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    let some_closure = || {
        println!("Vector is {:?}", x);
    };
    println!("Vector is {:?}", x);
    some_closure();
}
```

In this case, the closure doesn't require explicit input parameters. Instead, since the closure captures its environment, all variables in the current scope will be available inside the closure. The Rust compiler determines how to use the variable inside the closure, as it's not specified in the closure's inputs and outputs. By default, if the value is not updated inside the closure, the Rust compiler assumes that the variable will be used as an immutable reference.

Since the variable inside the closure is used as an immutable reference, x remains accessible because it's borrowed by reference.

If we attempt to update the vector, Rust will rightly complain because the vector is borrowed as immutable, and we cannot change its value until the immutable reference goes out of scope.

Once some_closure is completed, the immutable reference is out of scope, and we can update the value of x:

```rust
fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    let some_closure = || {
        println!("Vector is {:?}", x);
    };
    x[1] = 10; // Compiler will complain
    println!("Vector is {:?}", x);
    some_closure();
    x[1] = 10; // Compiler will not complain
}
```

Closures in Rust differ from typical functions. A typical function gets a chance to execute only when it is called from the main function or another function. However, as soon as we declare a closure and assign it to a variable, since the variable is in scope, the definition of the closure is also tied to the variable in some sense. As a result, after the some_closure call is completed, the immutable reference ends, allowing us to modify or update the value of the vector.

**Borrowing with Mutable Reference**

Now let's see what happens when a closure uses a mutable reference to a variable:

```rust
fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    let mut some_closure = || {
        x.push(6);
        println!("Vector is {:?}", x);
    };
    some_closure();
}
```

Since the closure modifies the variable, the Rust compiler infers that the variable will be borrowed by the closure as a mutable reference. As a result, until some_closure is in scope, we cannot have an immutable reference to the same variable. Additionally, the variable some_closure remains in scope until it is used for the last time. Note that we need to declare the some_closure variable as mut because the closure it represents will update the value of a variable, requiring it to be mutable.

To illustrate this further, we can add a print statement before calling the closure:

```rust
fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    let mut some_closure = || {
        x.push(6);
        println!("Vector is {:?}", x);
    };
    println!("Vector is {:?}", x);
    some_closure();
}
```

This code results in an error because x is borrowed as mutable, and therefore, an immutable borrow is not allowed.

Similarly, updating the vector before calling the closure is also not allowed because having multiple mutable borrows to the same variable violates Rust's ownership rules.

**Moving a Value into a Closure**

```rust
fn main() {
    let mut x = vec![1, 2, 3, 4, 5];
    let mut some_closure = || {
        let y = x;
    };
    some_closure();
    println!("X is {:?}", x);
    println!("Y is {:?}", y);
}
```

When we assign the value of x to y using let y = x;, the Rust compiler infers that the value is being moved or the ownership is being transferred. Therefore, it uses the original values and not references. Since the ownership has been transferred inside the closure, using x in the print statement is invalid because x is no longer in scope and has lost ownership.

Furthermore, the y variable is also dropped because its scope was limited to the body of the closure.

## Function Types

Function pointer types are used to refer to functions whose identities are not necessarily known at compile time. Unlike referencing data values, a function pointer points to executable code within memory. By using function pointers, we can define variables that have function types, meaning they can point to different functions, and we can change the pointer at runtime to point to different functions.

Here's an example in Rust:

```rust
fn main() {
    let mut f = max;
    println!("The maximum of the two values is {}", f(10, 25));
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}
```

**Uses of Function Types**

**Function Types as Parameters to Functions**

Function types can be used as parameters to other functions. Here's an example:

```rust
fn main() {
    display(max, 10, 25);
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn display(f: fn(i32, i32) -> i32, x: i32, y: i32) {
    println!("The maximum number is {}", f(x, y));
}
```

In this example, the display function takes a function of type fn(i32, i32) -> i32 as a parameter. It then calls the provided function (f) with the given values (x and y).

## Iterators

An iterator represents a sequence of items and provides methods for retrieving and manipulating those items one at a time. When you're looping over something, you're likely already using an iterator. If you're transforming collections, you should consider using iterators as well. Essentially, whenever you use a for loop in your program, you're likely interacting with some kind of iterator. Iterators in Rust are typically based on the .iter function.

Here's an example of creating an iterator in Rust:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();
}
```

In this code, the iter variable is an iterator that allows us to iterate through all the values of the numbers vector. By default, the iterator doesn't perform any useful actions. It becomes relevant when we apply operations to it. In other words, iterators have no effects until you call methods that consume the iterator. For this reason, they're sometimes referred to as being "lazy."

We can print the value of the iterator to see the captured values:

```rust
println!("The value of iter is {:?}", iter);
```

Output:

The value of iter is Iter([1, 2, 3, 4, 5])

All the values of the vector are captured by the iterator.

One of the key functions related to iterators is the .next function. It returns an Option enum, which can be either Some or None. The first call to next will return Some with the value of 1.

```rust
println!("{:?}", iter.next());
```

Output: Some(1)

Each subsequent call to next will return the next value in the iterator. When there are no more values in the vector, next will return None.

There are many other useful functions available for iterators.

**Useful Functions for Iterators**

**any function**

The any function uses a closure to determine if a certain condition is matched by any of the values in the iterator. If any value satisfies the condition, it returns true; otherwise, it returns false.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut check = numbers.iter().any(|&x| x > 4);
    println!("Status of any value greater than 4 in `numbers` is {}", check);
}
```

Output: Status of any value greater than 4 in 'numbers' is true

**all function**

The all function checks if all values in the iterator satisfy a given condition.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut check = numbers.iter().all(|&x| x > 4);
    println!("Status of all values greater than 4 in `numbers` is {}", check);
}
```

**find function**

The find function searches for an element in an iterator that satisfies a given condition. It takes a closure as an argument and returns an Option enum with the value inside it, wrapped in a reference. The closure is applied to each element of the iterator, and if the condition is met, find returns Some with the element. find stops processing as soon as the condition in the closure becomes true for a certain element.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut check = numbers.iter().find(|&&x| x > 4);
    println!("Status of any value greater than 4 in `numbers` is {:?}", check);
}
```

In this example, &&x is used because .iter uses references, so the values given to the find function are behind references. The closure also uses references to access the actual values.

The any and find functions are similar but have slight differences. The any function returns a boolean value (true or false) if the condition is met by any value, while the find function returns the value wrapped in an Option enum.

**position function**

The position function returns the index of an element that matches a given condition. It accepts a closure and checks for a certain condition. The function stops processing as soon as the condition becomes true for a certain element. The output of the function is an Option enum.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let mut check = numbers.iter().position(|&x| x > 4);
    println!("Index of a value greater than 4 in `numbers` is {:?}", check.unwrap());
}
```

Sometimes we want to find the position from the extreme right:

```rust
let mut check = numbers.iter().rposition(|&x| x > 4);
```

**Functions to Compute Basic Statistics for Vectors**

**max function**

```rust
let max_num = numbers.iter().max();
println!("Maximum number in `numbers` is {:?}", max_num.unwrap());
```

The output of this function is an Option enum, so we use unwrap to extract the value. If several elements are equally maximum, the last element is returned.

Similar to max, we also have a min function.

**reverse function**

```rust
let reversed = numbers.iter().rev();
```

If you print the value of reversed, it won't be in reverse order. However, if you change iter to iter.next, you'll get the first value from the end of the iterator.

**Modifying and Collecting Values with Iterators**

In Rust, iterators provide powerful functionality for modifying and collecting values. One such function is collect, which transforms an iterator into a collection such as an array, vector, or another type of collection. The collect function is widely used in various contexts.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let filtered_numbers = numbers.iter().filter(|&x| *x > 5).collect::<Vec<&i32>>();
    println!("Filtered values: {:?}", filtered_numbers);
}
```

The filter function creates an iterator that uses a closure to determine whether an element should be yielded or not. Therefore, the output of this function is also an iterator. The values returned to us are references to the original elements. If you want the values themselves, you can use into_iter instead of iter.

```rust
let filtered_numbers = numbers.into_iter().filter(|x| *x > 5).collect::<Vec<i32>>();
```

Using into_iter takes ownership of the vector, so you can no longer use the numbers variable afterwards. If you need to use the original vector later on, you can make a copy of it:

```rust
let cloned_numbers = numbers.clone();
```

Another useful function related to iterators is map.

**map function**

The map function takes a closure and creates an iterator that calls that closure on each element, modifying the values. It then returns another iterator with the modified values. For example, if you want to multiply all the values in a vector by 2, you can use map:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let cloned_numbers = numbers.clone();
    let filtered_numbers = numbers.into_iter().filter(|x| *x > 5).collect::<Vec<i32>>();
    println!("Filtered values: {:?}", filtered_numbers);
    let mapped_numbers = cloned_numbers.iter().map(|x| *x * 2).collect::<Vec<i32>>();
    println!("Mapped values: {:?}", mapped_numbers);
}
```

The mapped values can be further filtered and collected using other iterator functions.

**Important Point to Note**

If we look at the line of code where the filter function is first used with iter, we can see that the input is behind two references (check on your IDE). However, in the line where filter is used together with map, the input to the filter function is behind a single reference. Therefore, we don't need to mention an additional & before the variable x. This means that the input type may change depending on the functions used after it. It's important to pay special attention to such cases by examining the types of the inputs. In particular, try to convert the type to a single reference by either adding or removing the & sign before the variable name. This will help you correctly interpret the values and references.
