# Rust Ownership System Learning Guide

This folder contains examples to help you understand Rust's ownership system, with comparisons to TypeScript concepts you're already familiar with.

## Files Overview

### 1. `01_basic_ownership.rs`

-   **Core Concept**: Ownership transfer and the move semantics
-   **TypeScript Comparison**: Shows how Rust differs from TypeScript's reference-based model
-   **Key Topics**:
    -   Variable ownership for primitives vs complex types
    -   Ownership transfer with assignment
    -   Cloning vs moving
    -   Function ownership patterns

### 2. `02_borrowing_references.rs`

-   **Core Concept**: Borrowing and references to avoid ownership transfer
-   **TypeScript Comparison**: Similar to passing objects by reference
-   **Key Topics**:
    -   Immutable borrowing (`&`)
    -   Mutable borrowing (`&mut`)
    -   Borrowing rules (compiler enforced)
    -   Dangling references prevention
    -   String slices (`&str`)

### 3. `03_practical_patterns.rs`

-   **Core Concept**: Real-world ownership patterns
-   **TypeScript Comparison**: Common programming patterns translated to Rust
-   **Key Topics**:
    -   Struct ownership
    -   Vector/array patterns
    -   Option and Result types
    -   Lifetime annotations
    -   Common ownership solutions

## How to Run Examples

Each file is a standalone Rust program. To run any example:

```bash
# Method 1: Copy content to main.rs and run
cargo run

# Method 2: Run specific file directly
rustc src/ownership-system/01_basic_ownership.rs && ./01_basic_ownership.exe
```

## Key Ownership Rules (Rust's Core Philosophy)

### 1. Ownership Rules

-   Each value in Rust has a single owner
-   When the owner goes out of scope, the value is dropped
-   Assignment transfers ownership (for non-Copy types)

### 2. Borrowing Rules

-   You can have either:
    -   One mutable reference (`&mut`)
    -   Any number of immutable references (`&`)
-   References must always be valid

### 3. TypeScript vs Rust Comparison

| TypeScript Concept  | Rust Equivalent           | Key Difference                              |
| ------------------- | ------------------------- | ------------------------------------------- |
| `let obj = {x: 1}`  | `let obj = Struct {x: 1}` | Rust: Ownership moves on assignment         |
| Function parameters | Function parameters       | Rust: Ownership transfers unless borrowed   |
| Object references   | `&` references            | Rust: Compiler enforces borrowing rules     |
| `null/undefined`    | `Option<T>`               | Rust: No null references, explicit handling |
| `try/catch`         | `Result<T, E>`            | Rust: Errors are values, not exceptions     |

## Learning Path

1. **Start with** `01_basic_ownership.rs` - Understand the fundamental concept
2. **Then** `02_borrowing_references.rs` - Learn how to work with data without taking ownership
3. **Finally** `03_practical_patterns.rs` - Apply ownership concepts to real scenarios

## Common Mistakes and Solutions

### Problem: "Value used here after move"

**Solution**: Use borrowing (`&`) instead of taking ownership

### Problem: "Cannot borrow as mutable more than once"

**Solution**: Ensure only one mutable reference exists at a time

### Problem: "Dangling reference"

**Solution**: Return owned data instead of references to local variables

## Tips for TypeScript Developers

-   Think of Rust's ownership as "manual memory management with compiler help"
-   Use `&` (borrowing) like you'd use object references in TypeScript
-   Embrace the compiler errors - they're teaching you Rust's safety guarantees
-   Start with explicit cloning, then optimize with borrowing once comfortable

## Next Steps

After mastering ownership, move on to:

-   Traits and generic programming
-   Pattern matching
-   Concurrency and threads
-   Advanced lifetime annotations
