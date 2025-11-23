mod ownership_system;

fn main() {
    println!("Hello, world!");

    /* Call the main function from basic_ownership.rs */
    println!("\n=== Calling Ownership System Demo ===");
    ownership_system::basic_ownership::basic_ownership_demo();
}
