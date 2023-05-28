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
