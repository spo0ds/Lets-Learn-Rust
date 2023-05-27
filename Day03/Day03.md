## Control Structures

Control structures are essential elements in programming that allow for the analysis of variables and the execution of code based on specific conditions. In Rust, the basic control structures are conditionals and loops.

**If-Else ladder**

The if-else ladder is used when we need to execute a block of statements or a code segment only when certain conditions are met.

```rust
fn main() {
    let marks = 90;
    if marks >= 90 {
        println!("You got A*");
    } else if marks >= 80 {
        println!("You got A");
    } else if marks >= 70 {
        println!("You got B+");
    } else if marks >= 60 {
        println!("You got B");
    } else if marks >= 50 {
        println!("You got C");
    } else if marks >= 40 {
        println!("You got D");
    } else {
        println!("You failed");
    }
}
```

**If-Let**

The if let construct allows us to handle a specific condition when pattern matching is involved. It can be used to simplify code and handle specific cases effectively.

```rust
let value = if marks > 90 {
    println!("We're proud of you");
    "Excellent"
} else {
    println!("You can aim for Excellent");
    "Great"
};
```

The return value of the if body is the statement without a semicolon at the end, and it should be of the same type.

**Match Statement**

The match statement is a control flow operator in Rust used to transfer control to a particular block of code based on the value of a variable being tested. It is similar to the switch statement found in other programming languages.

```rust
fn main() {
    let marks = 90;
    match marks {
        90..=100 => println!("You got A*"),
        80..=89 => println!("You got A"),
        _ => println!("Work harder"),
    }
}
```

The match statement allows us to handle different cases and execute specific code based on the matched pattern.

## Loops

Loops are used to repeatedly execute a code block until a certain condition is met.

**Loops with no condition**

```rust
fn main() {
    loop {
        println!("Infinite loop");
    }
}
```

This type of loop may not be needed in most cases. Typically, there is a statement within the loop that allows it to be exited once a specific condition is met.

**While loops**

A while loop repeats a specific block of code for an unknown number of times. It is useful when we don't know how many times a user may input a value before guessing correctly, for example.

```rust
fn main() {
    let num = 5;
    let mut guess = false;

    println!("Guess the secret number between 1 and 10");

    while guess != true {
        let mut guessed_string = String::new();
        std::io::stdin()
            .read_line(&mut guessed_string)
            .expect("Failed to read input");
        let guessed_num: u8 = guessed_string.trim().parse().expect("Invalid input");
        if num == guessed_num {
            println!("You guessed the number correctly");
            guess = true;
        } else {
            println!("Try Again");
        }
    }
}
```

**For loops**

A for loop is used when we know the exact number of times the loop will execute. It is commonly used to iterate through the elements of arrays or vectors.

```rust
fn main() {
    let num = vec![10, 20, 30, 40];
    for i in 0..num.len() {
        println!("Array index {} has value {}", i, num[i]);
    }
}
```

We can also iterate using a different style:

```rust
fn main() {
    let num = vec![10, 20, 30, 40];
    for i in num {
        println!("Array has value {}", i);
    }
}
```

In this case, the values are printed in sequential order as they appear in the vector. However, note that the vector num is consumed inside the loop. If we want to access it outside the loop, an error will occur. To avoid this, we can use the iter function:

```rust
fn main() {
    let num = vec![10, 20, 30, 40];
    for i in num.iter() {
        println!("Array has value {}", i);
    }
}
```

Alternatively, we can pass a reference to the num variable:

```rust
for i in &num {
    println!("Array has value {}", i);
}
```

When using iter or a reference, the type of the variable i will be &i32, indicating an immutable reference to the values. If we need a mutable reference to the values, we can use iter_mut:

```rust
fn main() {
    let mut num = vec![10, 20, 30, 40];
    for i in num.iter_mut() {
        *i += 5;
        println!("Array has value {:?}", i);
    }
}
```

Output:

    Array has value 15
    Array has value 25
    Array has value 35
    Array has value 45

Here, the `*` operator is called a dereference operator. When using a mutable reference, the variable itself acts as a pointer, not the actual value. To update the actual value pointed to by the variable, we use the `*` operator.

In this case the variable i is set to type `&mut i32`.This gives us useful hint that instead of using this function, we can also use `&mut num` instead of num.iter_mut(.)

## Break and Continue

**Break Statement**

The break statement is used to stop the execution of a loop. When encountered inside a loop, the break statement immediately terminates the loop, and control resumes at the next statement following the loop.

```rust
fn main() {
    let mut num = 1000;
    loop {
        num = num - 1;
        if num % 13 == 0 {
            break;
        }
    }
    println!("The highest number is {}", num);
}
```

Output:
The highest number is 988

**Continue Statement**

The continue statement is used to skip the current iteration of a loop and proceed with the next iteration. It allows for the skipping of certain statements within the loop, and the loop continues with the next iteration.

```rust
fn main() {
    for number in 1..=10 {
        if number % 2 == 0 {
            continue;
        }
        println!("Odd number: {}", number);
    }
}
```

The continue statement is useful when we want to skip specific iterations based on certain conditions.

The break statement can also be used to return a value from a loop. However, this is only valid with simple loops such as for and while loops.

```rust
fn main() {
    let numbers = [1, 3, 5, 2, 7, 9, 4, 6, 8];
    let mut i = 0;
    let first_even = loop {
        if numbers[i] % 2 == 0 {
            break numbers[i];
        }
        i = i + 1;
    };
    println!("The first even number in the array is {}", first_even);
}
```

In this example, the loop iterates through the elements of the numbers array until it finds the first even number. Once the even number is found, the loop is terminated using break, and the value of the even number is returned.

## Coding Time

If you are not familiar with the concept of a stack, I would recommend watching a tutorial on YouTube to gain a better understanding. Once you have familiarized yourself with the stack data structure, you can proceed with implementing it in Rust.

Implementing a stack in Rust will provide you with a valuable learning experience regarding the ownership rules in Rust. It will help you grasp the concepts of ownership, borrowing, and lifetimes, which are fundamental to writing safe and efficient Rust code.

<details>
  <summary>Click to see my simple stack implementation</summary>
  
  [stack](https://github.com/spo0ds/Lets-Learn-Rust/tree/main/Day03/Stack/src/main.rs)
</details>
