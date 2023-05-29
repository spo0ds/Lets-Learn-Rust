## Structure

In Rust, a structure is a data type that allows you to group items of possibly different types into a single cohesive type. It provides a way to store information about something that is more complex than a single number, character, or boolean value, and more structured than an array. For example, consider a student who can be defined by their name, age, GPA, and other attributes. Each of these pieces of information can be labeled with a descriptive title and combined to form a whole, referred to as a structure.

To define a structure in Rust, you declare it using the struct keyword, followed by the name of the structure and its fields. Here's an example:

```rust
struct Person {
    citizenship: String,
    name: String,
    age: u32,
    gender: char,
    salary: u32,
}
```

Structures are typically defined outside of the main function. The order in which the values are assigned to the fields does not matter.

To access the fields of a structure, you use the dot (.) operator followed by the field name. For example:

```rust
let person1 = Person {
    citizenship: String::from("123A"),
    name: String::from("James Newton"),
    age: 25,
    gender: 'M',
    salary: 40_000,
};

println!("Person1 age is {}", person1.age);
```

Once data is organized into structures, you can define functions that operate on instances of specific structures. To better organize these functions within the context of a specific structure, Rust provides an implementation block for the structure. This block contains the relevant functions that operate on the type of the structure, using the object or instance of the structure as the receiver for the function.

Here's an example of defining a function within an implementation block for the Person structure:

```rust
impl Person {
    fn compute_taxes(&self) -> f32 {
        (self.salary as f32 / 3.0) * 0.5
    }
}
```

The impl keyword is used to declare the implementation block, followed by the name of the structure. The function compute_taxes takes &self as the first parameter, which is a reference to the structure object. You can then call this function on an instance of the Person structure, like this:

```rust
println!("The taxes on person {} is ${}.", person1.name, person1.compute_taxes());
```

In Rust, it is useful to have an initializing function that sets the initial values of a new structure with some useful defaults. This function is typically named new and is defined within the implementation block. It returns a new instance of the structure.

Here's an example of defining a new function for the Person structure:

```rust
impl Person {
    fn new() -> Self {
        Person {
            citizenship: String::from("123A"),
            name: String::from("James Newton"),
            age: 25,
            gender: 'M',
            salary: 40_000,
        }
    }
}
```

The Self keyword within the function signature indicates that the function returns an instance of the structure itself. You can then create new instances of the Person structure by calling this function, like this:

```rust
let person2 = Person::new();
println!("{} age is {}", person2.name, person2.age);
```

Rust also provides a convenient way to initialize a structure with the values of another existing structure using the field initialization shorthand. This allows you to specify some fields explicitly and use the remaining fields from another structure.

Here's an example of using the field initialization shorthand:

```rust
let person3 = Person {
    name: String::from("Nathan"),
    age: 50,
    ..person1
};
```

The ..person1 syntax indicates that the remaining fields in person3 should have the same values as the corresponding fields in person1.

By default, structures in Rust are immutable, meaning their values cannot be changed once assigned. However, if you want to create a mutable structure, you can use the mut keyword when defining the structure variable.

Here's an example of creating a mutable structure and updating its field:

```rust
let mut person1 = Person {
    citizenship: String::from("123A"),
    name: String::from("James Newton"),
    age: 25,
    gender: 'M',
    salary: 40_000,
};

person1.name = String::from("Harry");
println!("Person 1 name is {}", person1.name);
```

In addition to structures, Rust also provides tuple structures. A tuple structure is similar to a tuple but with the key difference that it has a name associated with it.

**Tuple Structure**

Here's an example of defining a tuple structure:

```rust
struct Numbers(i32, i32);
```

You can access the fields of a tuple structure using their indexes, like this:

```rust
let num = Numbers(10, 15);
println!("Field values are {} and {}", num.0, num.1);
```

You can also define functions within an implementation block for a tuple structure. These functions can operate on instances of the structure and access the fields using their indexes.

Here's an example of defining a function to determine the greater value in a tuple structure:

```rust
impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 > self.1 {
            self.0
        } else {
            self.1
        }
    }
}
```

You can then call this function on an instance of the tuple structure, like this:

```rust
let num = Numbers(10, 15);
println!("The greater among the two is {}", num.greater());
```

## Traits and Default Implementations

In Rust, traits are used to define shared behavior among different types. They provide an abstract definition of functionality that a particular type can have and share with other types. A trait consists of function signatures that can be called on types that implement the trait. By defining traits, we can group together function signatures to define a set of behaviors.

To illustrate this, let's define two structs: Person and Student.

```rust
struct Person {
    citizenship: String,
    name: String,
    age: u32,
    gender: char,
    salary: u32,
}

struct Student {
    citizenship: String,
    name: String,
    age: u32,
    sex: char,
}
```

Next, we'll define a trait called GeneralInfo that provides general information about an instance of the Student or Person struct.

```rust
trait GeneralInfo {
    fn info(&self) -> (&str, u32, char);
    fn country_info(&self) -> &str;
}
```

Inside the trait, we define function signatures without their detailed implementations.

Now, outside the trait body, we can implement the trait for specific types, such as the Person struct.

```rust
impl GeneralInfo for Person {
    fn info(&self) -> (&str, u32, char) {
        (&self.name, self.age, self.gender)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}
```

The function signatures inside the impl block should match the ones defined in the trait. When implementing a trait for a specific type, all the required functions of the trait must be defined. In this case, we added a & before self because the return type is a string reference.

Similarly, we can implement the GeneralInfo trait for the Student struct since it also has name, age, and sex information.

```rust
impl GeneralInfo for Student {
    fn info(&self) -> (&str, u32, char) {
        (&self.name, self.age, self.sex)
    }

    fn country_info(&self) -> &str {
        &self.citizenship
    }
}
```

Now, let's use these trait functions inside the main function.

```rust
fn main() {
    let person1 = Person {
        citizenship: String::from("123A"),
        name: String::from("James Newton"),
        age: 25,
        gender: 'M',
        salary: 40_000,
    };

    let student1 = Student {
        citizenship: String::from("432Z"),
        name: String::from("Kristina Bale"),
        age: 20,
        sex: 'F',
    };

    println!("General info: {:?}", person1.info());
    println!("Country info: {:?}", student1.country_info());
}
```

**Default Implementations**

Now, let's make some changes to the GeneralInfo trait by adding a default implementation for the country_info function directly inside the trait.

```rust
trait GeneralInfo {
    fn info(&self) -> (&str, u32, char);

    fn country_info(&self) -> &str {
        "Not implemented"
    }
}
```

This is called a default implementation for a function. If a type doesn't provide its own implementation for a function defined inside the trait, the default implementation will be used.

In the example, we comment out the country_info implementation in the Student struct. The compiler will not complain because, with the default implementation, we no longer need to provide an implementation for all the functions of a trait.

```rust
fn main() {
    let student1 = Student {
        citizenship: String::from("432Z"),
        name: String::from("Kristina Bale"),
        age: 20,
        sex: 'F',
    };

    println!("Country info: {:?}", student1.country_info());
}

impl GeneralInfo for Student {
    fn info(&self) -> (&str, u32, char) {
        (&self.name, self.age, self.sex)
    }

    // fn country_info(&self) -> &str {
    //     &self.citizenship
    // }
}
```

Now, when calling student1.country_info(), the default implementation from the trait will be used since there is no specific implementation provided for the Student struct.

## Functions with Traits

In this example, we will explore how functions defined within an implementation block of a trait for a specific type can call other functions within the same trait.

Let's start by defining a structure called Data, which contains a vector of numbers.

```rust
struct Data {
    num: Vec<i32>,
}
```

Next, we'll define a trait called BasicStats, which includes functions to calculate the mean and variance.

```rust
trait BasicStats {
    fn mean(&self) -> f32;
    fn variance(&self) -> f32;
}
```

Now, we can implement the BasicStats trait for the Data type.

```rust
impl BasicStats for Data {
    fn mean(&self) -> f32 {
        let mut sum = 0;
        for i in self.num.iter() {
            sum = sum + *i;
        }
        sum as f32 / self.num.len() as f32
    }

    fn variance(&self) -> f32 {
        let mean_value = self.mean();
        let mut sum_squared_diff: f32 = 0.0;
        for i in self.num.iter() {
            sum_squared_diff += (*i as f32 - mean_value) * (*i as f32 - mean_value);
        }
        sum_squared_diff / self.num.len() as f32
    }
}
```

To calculate the variance, we subtract each value from the mean of all the values in the vector and square the result. The squared differences are then summed together. This sum of squared differences is divided by the total number of values to obtain the variance. We store the mean value by calling the mean function for the same instance (self.mean()).

Finally, let's call these functions within the main function.

```rust
fn main() {
    let my_data = Data {
        num: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    };
    println!("The mean of the data is {}", my_data.mean());
    println!("The variance of the data is {}", my_data.variance());
}
```

When running the program, it will output the mean and variance of the data. By implementing the BasicStats trait for the Data type, we can easily calculate these statistical measures for instances of the Data struct.

## Enums

Enums are a data type that consists of a set of named values called elements. They are useful for representing a group of related options or variations. For example, an enum representing colors may include variants such as Red, Blue, Green, and Orange.

Let's consider a scenario in which a company organizes an event, and participants attend the event using different modes of transportation, such as cars, trains, and planes. Now, after the event, the company plans to reimburse the travel allowances to the participants based on their respective travel modes.

To calculate the travel allowance, we can use enums. First, we define an enum called Conveyance, which represents the various modes of transportation.

```rust
enum Conveyance {
    Car,
    Train,
    Aeroplane,
}
```

Next, we declare a variable of the enum type within the main function.

```rust
fn main() {
    let participant_1 = Conveyance::Car;
}
```

To implement the travel allowance function for this specific enum, we follow the same approach as we did with functions associated with structures.

```rust
impl Conveyance {
    fn travel_allowance(&self, miles: i32) -> f32 {
        let allowance = match self {
            Conveyance::Car => miles as f32 * 14.0 * 2.0,
            Conveyance::Train => miles as f32 * 18.0 * 2.0,
            Conveyance::Aeroplane => miles as f32 * 30.0 * 2.0,
        };
        allowance
    }
}
```

Now, let's use this function to compute the travel allowance for different participants.

```rust
fn main() {
    let participant_1 = Conveyance::Car;
    let participant_2 = Conveyance::Train;
    let participant_3 = Conveyance::Aeroplane;

    println!(
        "Participant 1 has a travel allowance of {}",
        participant_1.travel_allowance(50)
    );
}
```

We can also attach additional data to the enum variants instead of defining them separately. In our case, we can update the enum to include the miles associated with each variant.

```rust
enum Conveyance {
    Car(i32),
    Train(i32),
    Aeroplane(i32),
}
```

When declaring instances of the enum type within the main function, we can specify the miles associated with each variant.

```rust
let participant_1 = Conveyance::Car(50);
let participant_2 = Conveyance::Train(60);
let participant_3 = Conveyance::Aeroplane(70);
```

Since the miles information is already captured by the enum itself, we no longer need to mention it explicitly when calling the travel_allowance function.

```rust
fn main() {
    let participant_1 = Conveyance::Car(50);
    let participant_2 = Conveyance::Train(60);
    let participant_3 = Conveyance::Aeroplane(70);

    println!(
        "Participant 1 has a travel allowance of {}",
        participant_1.travel_allowance()
    );
}
```

Lastly, let's update the travel allowance function.

```rust
fn main() {
    let participant_1 = Conveyance::Car(50);
    let participant_2 = Conveyance::Train(60);
    let participant_3 = Conveyance::Aeroplane(70);

    println!(
        "Participant 1 has a travel allowance of {}",
        participant_1.travel_allowance()
    );
}
```

Enums can be used in various ways to achieve smart solutions. For instance, we can create a vector containing different types of data using enums, even though vectors typically require elements of the same type.

```rust
enum Value {
    Integer(i32),
    Float(f32),
}
```

We can declare a vector with elements of the enum variant.

```rust
fn main() {
    let values = vec![Value::Integer(7), Value::Float(3.5)];
}
```

The type of the vector is set to the enum type, Value.

```rustfn main() {
    let values = vec![Value::Integer(7), Value::Float(3.5)];
    println!("The value of the integer is {}", values[0]);
    println!("The value of the float is {}", values[1]);
}
```

However, when attempting to print the values, an error occurs: Value doesn't implement std::fmt::Debug. To ensure that the enum uses the required std::fmt::Display trait for printing, we add #[derive(Debug)] just before the enum definition.

```rust
#[derive(Debug)]
enum Value {
    Integer(i32),
    Float(f32),
}
```

After making this change, the output will be as follows:

The value of the integer is Integer(7)
The value of the float is Float(3.5)

We can add additional logic to determine the exact type of the value.

```rust
for value in values.iter() {
    match value {
        Value::Integer(num) => println!("The value is an integer with a value of {}", num),
        Value::Float(num) => println!("The value is a float with a value of {}", num),
    }
}
```

The output will be:

The value is an integer with a value of 7
The value is a float with a value of 3.5

It's important to note that the requirement for a vector to have elements of a single type still holds in our case, as the vector contains values of the enum type, Value.

## Generics

Generics enable us to write flexible and reusable code by abstracting over types. Instead of specifying concrete types, we can define functions and data structures in terms of generic types that are determined later based on the actual data they receive.

To illustrate this, let's consider a scenario where we need to compute the square of a given number. Instead of writing separate functions for integers and floats, which would involve duplicating code, we can use generics to write a single function that works for both types.

```rust
fn main() {
    println!("The square of the integer {} is {}.", 3, square(3));
    println!("The square of the float {} is {}.", 3.5, square(3.5));
}

fn square<T>(x: T) -> T {
    x * x
}
```

However, when we write this code, the compiler notifies us that it cannot multiply T by T. Since T can be any type, Rust cannot perform multiplication for all possible cases. We need to provide the compiler with more information about the possible types that T can be by specifying certain traits for the type.

```rust
fn square<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
    x * x
}
```

This code tells the Rust compiler that we are using a generic type T with the restriction that it must implement the Mul trait (which represents multiplication) and be Copy. The Output = T constraint in the Mul trait indicates that the result of the multiplication should be of type T.

To make the code more readable when there are multiple trait restrictions, Rust provides an alternate syntax using the where clause.

```rust
fn square<T>(x: T) -> T
where
    T: std::ops::Mul<Output = T> + Copy,
{
    x * x
}
```

Let's explore another example of generics using structures. Suppose we want to define a simple structure to represent a point, where a point is defined by two values. We can define the struct with a generic type.

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p1 = Point { x: 3, y: 5 };
    let p2 = Point { x: 3.5, y: 5.7 };
}
```

This code works fine when x and y have the same type. However, if we want to allow different types for x and y, we can introduce another generic field using a different generic parameter.

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 3, y: 5 };
    let p2 = Point { x: 3.5, y: 5.7 };
    let p3 = Point { x: 3, y: 5.7 };
}
```

Now the compiler is satisfied, and we can define functions on the Point struct. For example, let's define a function to print the values of the point.

```rust
impl<T: std::fmt::Debug, U: std::fmt::Debug> Point<T, U> {
    fn print_point(&self) {
        println!(
            "The values of the point are x = {:?} and y = {:?}",
            self.x, self.y
        );
    }
}
```

Now we can use the print_point function to print the values of different points.

```rust
fn main() {
    let p1 = Point { x: 3, y: 5 };
    let p2 = Point { x: 3.5, y: 5.7 };
    let p3 = Point { x: 3, y: 5.7 };

    p1.print_point();
    p2.print_point();
    p3.print_point();
}
```

Output:

The values of the point are x = 3 and y = 5
The values of the point are x = 3.5 and y = 5.7
The values of the point are x = 3 and y = 5.7

Throughout our journey, we have consistently utilized the power of generics, starting with the vector data structure.

In Rust, there are two frequently used enums that serve specific purposes in certain situations. These enums are called Option and Result.

## Option Enum

The Option enum is commonly used when we need to handle the possibility of failure or the absence of a value within a program segment. It consists of two variants. The first variant is None, which signifies the absence or failure of a value. The second variant is Some, which is a tuple struct that wraps a value of type T, where T is a generic type.

```rust
fn main() {
    let mut disease: Option<String> = None;
    disease = Some(String::from("Diabetes"));

    match disease {
        Some(disease) => println!("You got {}", disease),
        None => println!("You're healthy"),
    }
}
```

Let's explore an example of using the Option enum to check the existence of values of different types.

```rust
fn main() {
    let s1: Option<&str> = Some("Hello");
    println!(
        "The value of s1 is {:?}, and s1 itself is {:?}",
        s1,
        s1.unwrap()
    );
}
```

Output:

The value of s1 is Some("Hello"), and s1 itself is "Hello"

We can also pass variables of the Option type as function parameters. Here's an example:

```rust
fn main() {
    let num = Some(6);
    if square(num) != None {
        println!(
            "The result of the square is {:?}",
            square(Some(num).unwrap()).unwrap()
        );
    } else {
        println!("Value missing");
    }
}

fn square(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}
```

In this code snippet, we define a function called square that takes an Option<i32> as a parameter. It squares the value if it exists and returns Some(result), otherwise it returns None. We then call this function and handle the options accordingly.

## Result Enum

The Result enum in Rust is another commonly used enum that is particularly useful for handling functions that can either produce a successful result (represented by the Ok variant) or encounter an error (represented by the Err variant). The Result enum is defined as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

The generic types T and E can represent any value type and error type, respectively.

Let's demonstrate the usage of the Result enum in a division function:

```rust
fn division(dividend: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    println!("Division: {:?}", division(50.0, 5.0).unwrap());
    println!("Division: {:?}", division(50.0, 0.0));
    println!("Division: {:?}", division(50.0, 10.0).unwrap());
}
```

In this code snippet, we define a division function that takes two f64 values as parameters and returns a Result<f64, String>. If the divisor is non-zero, the division is successful and we use the Ok variant to wrap the result. However, if the divisor is zero, we encounter an error and use the Err variant to wrap an error message represented by a String.

In the main function, we call the division function with different arguments and handle the Result accordingly. We can use the unwrap method to directly access the value of a successful division, but it will panic if the Result is an error.

The Result enum is extensively used by many built-in functions in Rust to handle potential errors and communicate the outcome of operations.

## Hash Maps

ChatGPT

Hash Maps are a fundamental data structure in Rust that allow us to store and retrieve values based on unique keys. Think of a hash map as a collection of key-value pairs, similar to a dictionary. The keys in a hash map are used to access their corresponding values.

To work with hash maps in Rust, we need to import the HashMap module from the standard library:

```rust
use std::collections::HashMap;
```

We can declare a new empty hash map as follows:

```rust
fn main() {
    let mut person: HashMap<&str, i32> = HashMap::new();
}
```

In this example, we're using string slices (&str) as keys and i32 as values for the hash map.

To insert values into the hash map, we can use the insert method, specifying the key and value:

```rust
person.insert("John", 30);
```

To access a specific value in the hash map, we can use the key as an index:

```rust
println!("The age is {} years old.", person["John"]);
println!("The age is {} years old.", person.get("John").unwrap());
```

We can check if a particular key exists in the hash map using the contains_key function, which returns a boolean value:

```rust
if person.contains_key("Kobe") {
    println!("Key exists.");
} else {
    println!("Key doesn't exist.");
}
```

Alternatively, we can use the get function to achieve the same result:

```rust
match person.get("John") {
    Some(_) => println!("Key exists."),
    None => println!("Key doesn't exist."),
}
```

To iterate through all the key-value pairs in the hash map, we can use a for loop with the iter method:

```rust
for (name, age) in person.iter() {
    println!("{} is {} years old.", name, age);
}
```

When inserting a value for an existing key, the previous value in the hash map will be overwritten. However, we can use the entry method along with or_insert to insert a value only if the key doesn't already exist:

```rust
person.entry("John").or_insert(31);
```

Let's consider a program that uses a hash map to store the frequency of numbers in a vector:

```rust
use std::collections::HashMap;

fn main() {
    let num = vec![1, 1, 2, 3, 2, 4, 5, 1, 7, 5, 4, 2];
    let mut freq: HashMap<i32, u32> = HashMap::new();

    for i in num.iter() {
        let value = freq.entry(*i).or_insert(0);
        *value += 1;
    }
    println!("{:?}", freq);
}
```

In this example, we iterate through the num vector and use the entry method to insert a value if it's not already present in the hash map. We then increment the frequency using \*value += 1. The final output displays the frequencies of the numbers.
