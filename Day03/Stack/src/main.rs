fn main() {
    println!("Enter the maximum capacity for the stack:");
    let mut capacity = String::new();

    std::io::stdin()
        .read_line(&mut capacity)
        .expect("Failed to read input");

    let capacity = capacity.trim().parse().expect("Invalid input");
    let mut numbers: Vec<i32> = Vec::with_capacity(capacity);
    let mut head: usize = 0;

    push(&mut numbers, &mut head, capacity);

    println!(
        "Top of the stack contains {}",
        top_of_the_stack(&numbers, head)
    );

    pop(&mut numbers, &mut head);

    pop(&mut numbers, &mut head);

    display(&numbers, head);
}

fn push(numbers: &mut Vec<i32>, head: &mut usize, capacity: usize) {
    println!("Enter the numbers to push into the stack separated by space");

    let mut user_num = String::new();

    std::io::stdin()
        .read_line(&mut user_num)
        .expect("Failed to read input");

    let parsed_space = user_num.trim();

    for i in parsed_space.split_whitespace() {
        let parsed_num: i32 = i.parse().expect("Invalid input");
        if *head == capacity {
            println!("Stack is full. Cannot push more elements.");
            return;
        }
        numbers.push(parsed_num);
        *head += 1;
    }
}

fn pop(numbers: &mut Vec<i32>, head: &mut usize) {
    if *head == 0 {
        println!("All elements have been removed from the stack");
        return;
    }

    *head -= 1;

    let removed_element = numbers.pop();

    if let Some(element) = removed_element {
        println!("The removed element from the stack is {}", element);
    }
}

fn display(numbers: &[i32], head: usize) {
    if head == 0 {
        println!("The stack is empty");
        return;
    }

    println!("The elements in the stack are:");

    for i in (0..head).rev() {
        println!("{}", numbers[i]);
    }
}

fn top_of_the_stack(numbers: &[i32], head: usize) -> i32 {
    if head == 0 {
        println!("The stack is empty");
        return 0;
    }

    numbers[head - 1]
}
