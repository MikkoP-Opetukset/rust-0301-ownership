#![allow(dead_code, unused_variables, unused_mut)]

/// Entry point for Chapter 4.1: What is Ownership?
///
/// Ownership is Rust's central feature that enables memory safety without needing
/// a garbage collector. This file explores the three rules of ownership, stack vs.
/// heap allocation, move vs. copy semantics, cloning, and how ownership transfers
/// into and out of functions.
fn main() {
    ownership_rules_and_scope();
    stack_vs_heap_memory();
    move_semantics();
    copy_semantics();
    clone_semantics();
    ownership_and_functions();
}

/// # The Three Rules of Ownership & Variable Scope
/// - Each value in Rust has an owner.
/// - There can only be one owner at a time.
/// - When the owner goes out of scope, the value is dropped.
fn ownership_rules_and_scope() {
    println!("\n{:=>80}", "");
    println!("ownership_rules_and_scope()\n");

    // A variable is valid from the point it is declared until the end of its current scope.
    {
        // `session_token` comes into scope here.
        let session_token = String::from("AUTH_891023_KEY");
        println!("Inside inner scope: {session_token}");

        // Do work with `session_token`...
    }
    // The inner scope is now over. Rust automatically calls `drop` for `session_token`
    // to return the heap memory to the operating system.

    // Trying to use `session_token` outside its scope will cause a compile-time error.
    // println!("Outside scope: {session_token}"); // Uncomment this line to see the error.
}

/// # Stack vs. Heap Allocation
/// - **Stack:** Fast, fixed size known at compile time. Stores primitive values and pointers.
/// - **Heap:** Slower allocation, dynamic size determined at runtime.
/// - String literals (`&str`) are hardcoded into the binary and stored on the stack/text segment.
/// - The `String` type allocates memory on the heap and allows dynamic modification.
fn stack_vs_heap_memory() {
    println!("\n{:=>80}", "");
    println!("stack_vs_heap_memory()\n");

    // String literal: Fixed, immutable, known at compile time.
    let literal_path: &str = "/var/log/system.log";

    // `String` type: Dynamic size, allocated on the heap at runtime.
    // Memory is allocated when `String::from` is called.
    let mut dynamic_path = String::from("/var/log/");
    println!("Initial path: {dynamic_path}");

    // Because it is heap-allocated and mutable, we can append data at runtime.
    dynamic_path.push_str("app.log");
    println!("Modified path: {dynamic_path}");
}

/// # Move Semantics (Heap Data)
/// - When assigning a dynamic heap object (`String`) to another variable, Rust copies
///   the pointer, length, and capacity stored on the stack, but does NOT copy the heap data.
/// - To prevent "double free" memory bugs when variables go out of scope, Rust invalidates
///   the original variable. This operation is called a **MOVE**.
fn move_semantics() {
    println!("\n{:=>80}", "");
    println!("move_semantics()\n");

    let original_payload = String::from("PAYLOAD_DATA_BUFFER");

    // Ownership of the heap memory is MOVED from `original_payload` to `transferred_payload`.
    let transferred_payload = original_payload;

    // `transferred_payload` is now the owner of the heap memory.
    println!("Transferred payload: {transferred_payload}");

    // `original_payload` is no longer valid because its contents were moved.
    // Rust prevents you from using invalidated references at compile time!
    // println!("Original payload: {original_payload}"); // Uncomment to see the use-after-move error.
}

/// # Copy Semantics (Stack Data)
/// - Types with a size known at compile time are stored entirely on the stack.
/// - These types implement the `Copy` trait (integers, booleans, floating points, chars,
///   and tuples containing only Copy types).
/// - Assigning a `Copy` variable creates a full value copy; the original variable remains valid.
fn copy_semantics() {
    println!("\n{:=>80}", "");
    println!("copy_semantics()\n");

    let item_count = 50;

    // `item_count` is copied into `assigned_count`. Both exist independently on the stack.
    let assigned_count = item_count;

    // Both variables are valid and can be used without issues.
    println!("Original count: {item_count}, Assigned count: {assigned_count}");

    // Tuples containing only primitive types also implement `Copy`.
    let coordinates = (61.4977, 21.7972); // Pori coordinates
    let location = coordinates;
    println!("Coordinates: ({}, {})", coordinates.0, coordinates.1);
}

/// # Clone Semantics (Deep Copy)
/// - If we DO want to deeply copy the heap data of a `String` (not just move ownership),
///   we can call the `.clone()` method.
/// - Cloning duplicates both the stack metadata and the heap allocation.
/// - Note: Cloning can be expensive in terms of CPU and memory performance.
fn clone_semantics() {
    println!("\n{:=>80}", "");
    println!("clone_semantics()\n");

    let primary_db_config = String::from("Host=localhost;Port=5432;User=admin;");

    // Explicitly cloning duplicates the heap memory.
    let secondary_db_config = primary_db_config.clone();

    // Since a deep copy was made, both variables own their separate heap memory and remain valid.
    println!("Primary Config:   {primary_db_config}");
    println!("Secondary Config: {secondary_db_config}");
}

/// # Ownership and Functions
/// - Passing a value to a function has the exact same semantics as assigning a value to a variable:
///   - Heap types (`String`) will be **moved** into the function parameter.
///   - Stack types (`i32`) will be **copied** into the function parameter.
/// - Returning values from a function transfers ownership back to the calling scope.
fn ownership_and_functions() {
    println!("\n{:=>80}", "");
    println!("ownership_and_functions()\n");

    let message = String::from("SYSTEM_WARN_OVERHEAT");

    // `message` is passed into the function. Ownership moves inside `consume_message`.
    consume_message(message);

    // `message` is no longer valid here because ownership was transferred into `consume_message`.
    // println!("Message after function call: {message}"); // Uncomment to see the error.

    let status_code = 500;

    // `status_code` implements `Copy`, so it is copied into `process_status_code`.
    process_status_code(status_code);

    // `status_code` remains completely valid here after the function call!
    println!("Status code in main scope: {status_code}\n");

    // --- Transferring ownership back via return values ---

    // `generate_report` creates a String and transfers ownership to `my_report`.
    let my_report = generate_report();
    println!("Generated report: {my_report}");

    // We can pass ownership into a function and get ownership back via the return value.
    let processed_report = audit_and_return_report(my_report);

    // `my_report` was moved into `audit_and_return_report`, so it can't be used here.
    // println!("Original report: {my_report}"); // Uncomment to see the error.

    // But `processed_report` now owns the returned String!
    println!("Processed report: {processed_report}");
}

// ========================================================================= //
// NOTE! Helper functions demonstrating parameter passing and return scopes.
// ========================================================================= //

/// Takes ownership of a heap-allocated `String`.
/// When this function ends, `data` goes out of scope and `drop` is called.
fn consume_message(data: String) {
    println!("Inside consume_message: '{data}'");
} // Memory for `data` is freed here!

/// Accepts an `i32` primitive. The value is copied onto this function's stack frame.
fn process_status_code(code: i32) {
    println!("Inside process_status_code: {code}");
} // Stack allocation cleared, original variable in caller function is unaffected.

/// Creates a `String` on the heap and transfers ownership to whichever variable
/// receives the function's return expression.
fn generate_report() -> String {
    let report = String::from("AUDIT_PASSED");

    // Returning the expression transfers ownership out of the function.
    report
}

/// Takes ownership of `report`, appends a timestamp, and returns ownership back to the caller.
/// (Notice how cumbersome taking and returning ownership is—this sets up the need for References!)
fn audit_and_return_report(mut report: String) -> String {
    report.push_str(" [AUDITED: OK]");

    report // Return expression transfers ownership back.
}
