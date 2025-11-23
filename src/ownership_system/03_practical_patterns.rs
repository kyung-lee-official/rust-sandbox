/*! Practical Ownership Patterns
 *! Real-world scenarios with TypeScript comparisons
 */

fn main() {
    println!("=== Practical Ownership Patterns ===");

    /* 1. STRUCT OWNERSHIP */
    println!("\n1. Struct Ownership:");

    let user = User {
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    /* In TypeScript: Objects are passed by reference
     * In Rust: Ownership moves unless we borrow */
    print_user(&user); /* Borrow to avoid moving ownership */
    println!("User still accessible: {}", user.name);

    /* 2. VECTOR OWNERSHIP */
    println!("\n2. Vector Ownership (like TypeScript arrays):");

    let mut numbers = vec![1, 2, 3, 4, 5];

    /* Borrow to iterate without taking ownership */
    for num in &numbers {
        println!("Number: {}", num);
    }

    /* Modify vector using mutable borrow */
    double_numbers(&mut numbers);
    println!("Doubled numbers: {:?}", numbers);

    /* 3. OPTION AND RESULT PATTERNS */
    println!("\n3. Option and Result Patterns:");

    let maybe_name: Option<String> = Some(String::from("Bob"));
    let no_name: Option<String> = None;

    /* Similar to TypeScript: if (name) { ... } */
    if let Some(name) = maybe_name {
        println!("Name found: {}", name);
    }

    /* Handle Result (like TypeScript try/catch) */
    let result = divide(10.0, 2.0);
    match result {
        Ok(value) => println!("Division result: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    /* 4. LIFETIME ANNOTATIONS (Advanced) */
    println!("\n4. Lifetime Annotations:");

    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    /* 5. COMMON OWNERSHIP MISTAKES AND SOLUTIONS */
    println!("\n5. Common Patterns and Solutions:");

    /* Problem: Want to use data after function call */
    let data = String::from("important data");
    let processed = process_and_return(data); /* Takes ownership and returns */
    println!("Processed data: {}", processed);

    /* Alternative: Borrow and return new data */
    let data2 = String::from("more data");
    let result2 = process_with_borrow(&data2);
    println!("Original: {}, Result: {}", data2, result2);
}

/* Struct definition */
struct User {
    name: String,
    email: String,
    active: bool,
}

fn print_user(user: &User) {
    println!("User: {} <{}>", user.name, user.email);
}

fn double_numbers(numbers: &mut Vec<i32>) {
    for num in numbers {
        *num *= 2; /* Dereference to modify the value */
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

/* Lifetime annotation: 'a means both parameters and return value have same lifetime */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn process_and_return(data: String) -> String {
    /* Takes ownership, processes, returns ownership */
    format!("Processed: {}", data)
}

fn process_with_borrow(data: &str) -> String {
    /* Borrows data, returns new String */
    format!("Processed: {}", data)
}

/* TypeScript Comparison Examples:
 *
 * // TypeScript equivalent patterns:
 *
 * // 1. Struct-like objects
 * interface User {
 *     name: string;
 *     email: string;
 *     active: boolean;
 * }
 *
 * function printUser(user: User): void {
 *     console.log(`User: ${user.name} <${user.email}>`);
 * }
 *
 * // 2. Array manipulation
 * let numbers = [1, 2, 3, 4, 5];
 * numbers.forEach(num => console.log(`Number: ${num}`));
 * numbers = numbers.map(num => num * 2);
 *
 * // 3. Optional values
 * let maybeName: string | null = "Bob";
 * if (maybeName) {
 *     console.log(`Name found: ${maybeName}`);
 * }
 *
 * // 4. Error handling
 * function divide(a: number, b: number): number {
 *     if (b === 0) throw new Error("Division by zero");
 *     return a / b;
 * }
 *
 * try {
 *     const result = divide(10, 2);
 *     console.log(`Division result: ${result}`);
 * } catch (e) {
 *     console.log(`Error: ${e}`);
 * }
 */
