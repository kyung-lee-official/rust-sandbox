/*! Borrowing and References
 *! TypeScript comparison: Similar to passing objects by reference
 */

fn main() {
    println!("=== Borrowing and References ===");

    /* 1. IMMUTABLE BORROWING (&) */
    println!("\n1. Immutable Borrowing (Read-only access):");

    let s1 = String::from("hello");

    /* In TypeScript: function len(str: string) { return str.length; }
     * In Rust: We borrow instead of taking ownership */
    let len = calculate_length(&s1); /* & creates a reference (borrow) */

    println!("The length of '{}' is {}.", s1, len); /* s1 still valid! */

    /* 2. MUTABLE BORROWING (&mut) */
    println!("\n2. Mutable Borrowing (Read-write access):");

    let mut s2 = String::from("hello");

    /* In TypeScript: function append(str: string, text: string) { str += text; }
     * In Rust: We need mutable reference to modify */
    append_text(&mut s2, " world");
    println!("After append: {}", s2);

    /* 3. BORROWING RULES */
    println!("\n3. Borrowing Rules (Compiler enforced):");

    let mut s3 = String::from("test");

    /* Rule 1: Multiple immutable references OK */
    let r1 = &s3;
    let r2 = &s3;
    println!("r1: {}, r2: {}", r1, r2);

    /* Rule 2: Only ONE mutable reference at a time */
    let r3 = &mut s3;
    /* let r4 = &mut s3; // ❌ Error: cannot borrow `s3` as mutable more than once */
    r3.push_str(" modified");

    /* Rule 3: Cannot mix mutable and immutable references */
    /* let r5 = &s3; // ❌ Error: cannot borrow `s3` as immutable because it's also borrowed as mutable */
    println!("r3: {}", r3);

    /* 4. DANGLE REFERENCES (Prevented by compiler) */
    println!("\n4. Dangling References (Compiler prevents):");

    /* This would cause a compile error:
     * let reference_to_nothing = dangle(); // ❌ Error! */
    let valid_reference = no_dangle();
    println!("Valid reference: {}", valid_reference);

    /* 5. SLICES - Special kind of reference */
    println!("\n5. Slices (View into data):");

    let s4 = String::from("hello world");
    let hello = &s4[0..5]; /* slice from index 0 to 4 */
    let world = &s4[6..11]; /* slice from index 6 to 10 */

    println!("Full: '{}'", s4);
    println!("First word: '{}'", hello);
    println!("Second word: '{}'", world);

    /* String slices are &str type (string references) */
    let first_word = first_word(&s4);
    println!("First word via function: '{}'", first_word);
}

fn calculate_length(s: &String) -> usize {
    /* s is a reference to a String (borrowed, not owned) */
    s.len()
    /* s goes out of scope, but since it's a reference, nothing is dropped */
}

fn append_text(s: &mut String, text: &str) {
    /* s is a mutable reference, so we can modify the original */
    s.push_str(text);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s /* Ownership is transferred out */
    /* This works because we return ownership, not a reference */
}

/* This would cause a compile error (dangling reference):
 * fn dangle() -> &String {
 *     let s = String::from("hello");
 *     &s // ❌ Error: returns reference to data that will be dropped
 * } */

fn first_word(s: &str) -> &str {
    /* &str is a string slice (reference to part of a string) */
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] /* Return entire string if no space found */
}
