/*! Ownership System - Basic Concepts
 *! Drawing parallels between TypeScript and Rust
 */

pub fn basic_ownership_demo() {
    println!("=== Ownership System Basics ===");

    /* 1. VARIABLE OWNERSHIP - Similar to TypeScript primitives */
    println!("\n1. Variable Ownership (Primitives):");

    /* In TypeScript: let x = 5; let y = x; (both have independent copies)
     * In Rust: Same behavior for primitive types (Copy trait) */
    let x = 5;
    let y = x; /* x is copied to y (both are independent) */
    println!("x = {}, y = {}", x, y); /* Both can be used */

    /* 2. STRING OWNERSHIP - Different from TypeScript! */
    println!("\n2. String Ownership:");

    /* In TypeScript: let s1 = "hello"; let s2 = s1; (both reference same string)
     * In Rust: Ownership moves! */
    let s1 = String::from("hello");
    let s2 = s1; /* Ownership MOVES from s1 to s2 */

    /* This would cause a compile error:
     * println!("s1 = {}", s1); // ❌ Error: s1 is no longer valid! */
    // println!("s1 = {}", s1); // ❌ This line would cause a compile error!
    println!("s2 = {}", s2); /* ✅ s2 owns the string now */

    /* 3. CLONING - Creating independent copies */
    println!("\n3. Cloning (like TypeScript object spread):");

    let s3 = String::from("world");
    let s4 = s3.clone(); /* Explicit copy (like {...s3} in TS) */

    println!("s3 = {}, s4 = {}", s3, s4); /* Both work! */

    /* 4. FUNCTION OWNERSHIP */
    println!("\n4. Function Ownership:");

    let s5 = String::from("function test");
    takes_ownership(s5); /* Ownership moves to the function */

    /* println!("s5 = {}", s5); // ❌ Error: s5 is no longer valid! */

    let s6 = gives_ownership(); /* Function gives ownership */
    println!("s6 = {}", s6);
}

fn takes_ownership(some_string: String) {
    println!("Inside function: {}", some_string);
    /* some_string is dropped here (memory freed) */
}

fn gives_ownership() -> String {
    let some_string = String::from("hello from function");
    some_string /* Ownership is returned */
}
