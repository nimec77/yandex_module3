# Yandex Module 3 - Rust Async Programming Examples

A collection of practical exercises and examples demonstrating asynchronous programming concepts in Rust using Tokio, Futures, and Smol runtimes.

## Overview

This project contains various tasks showcasing different aspects of async Rust programming, including:
- Custom Future implementations
- Timeout handling
- Task pools and concurrency
- TCP networking
- Serialization with custom validation
- Web server development with Actix-web

## Prerequisites

- Rust 2024 edition
- Cargo package manager

## Dependencies

- **tokio** (1.40) - Primary async runtime with full features
- **futures** (0.3) - Future combinators and utilities
- **smol** (2.0.2) - Alternative async runtime
- **waker-fn** (1.2.0) - Utilities for creating wakers
- **serde** (1.0) - Serialization/deserialization framework
- **serde_json** (1.0) - JSON support for serde
- **chrono** (0.4) - Date and time library
- **uuid** (1.18.1) - UUID generation and handling
- **thiserror** (2.0.17) - Error handling macros
- **actix-web** (4.12.0) - Web framework

## Project Structure

```
src/
├── main.rs          - Concurrent connection handler
└── bin/
    ├── task2.rs     - Custom waker implementation
    ├── task3.rs     - Custom Future implementation (WaitFor)
    ├── practice1.rs - Timeout wrapper for futures
    ├── practice2.rs - Task pool with bounded queue
    ├── task4.rs     - TCP client example
    ├── task5.rs     - Serialization with validation
    └── task6.rs     - Simple web server
```

## Examples and Usage

### Main Program: Concurrent Connection Handler

Handles multiple async connections concurrently using Tokio's spawn.

```bash
cargo run
```

**What it does:**
- Creates 10 concurrent async connections
- Spawns each as a separate Tokio task
- Ensures all connections complete in under 500ms

### Task 2: Custom Waker Implementation

Demonstrates manual Future polling with custom waker using smol runtime.

```bash
cargo run --bin task2
```

**Features:**
- Reads Cargo.toml asynchronously
- Manual polling loop with custom waker
- Thread parking/unparking for synchronization

### Task 3: Custom Future (WaitFor)

Implementation of a custom Future that waits for a specified duration.

```bash
cargo run --bin task3
```

**Implementation details:**
- Creates a custom `WaitFor` Future
- Uses thread-based timer and waker
- Demonstrates Future trait implementation from scratch

### Practice 1: Timeout Wrapper

Wraps any future with a timeout capability.

```bash
cargo run --bin practice1
```

**Examples:**
```rust
// Instant completion
let instant = async { 0 };
let result = timeouted_read(Duration::from_millis(123), instant).await;
// Output: Result(0)

// Completes before timeout
let wait100 = async {
    tokio::time::sleep(Duration::from_millis(100)).await;
    100
};
let result = timeouted_read(Duration::from_millis(123), wait100).await;
// Output: Result(100)

// Exceeds timeout
let wait150 = async {
    tokio::time::sleep(Duration::from_millis(150)).await;
    150
};
let result = timeouted_read(Duration::from_millis(123), wait150).await;
// Output: Timeout
```

### Practice 2: Task Pool

Bounded task pool with timeout for task creation.

```bash
cargo run --bin practice2
```

**Features:**
- Bounded MPSC channel (queue_size: 2)
- 100ms timeout for task creation when queue is full
- Demonstrates backpressure handling

**Usage example:**
```rust
let mut tasks = TaskPool::new(2);

// First two tasks succeed immediately
tasks.create(()).await; // Some(Ok(()))
tasks.create(()).await; // Some(Ok(()))

// Third task times out (queue full)
tasks.create(()).await; // None (after 100ms)

// Pull a task to free space
tasks.pull_task().await; // Some(())

// Now creation succeeds again
tasks.create(()).await; // Some(Ok(()))
```

### Task 4: TCP Client

Simple HTTP client using Tokio's async TCP.

```bash
cargo run --bin task4
```

**What it does:**
- Connects to google.com:80
- Sends HTTP/1.1 GET request
- Reads and prints response

### Task 5: Serialization with Validation

Advanced serde usage with custom serialization and validation.

```bash
cargo run --bin task5
```

**Features:**

**Product serialization:**
- Prices stored in kopecks (cents) as integers
- Optional category field (skipped if None)
- Internal ID excluded from serialization

**Order validation:**
- Email format validation during deserialization
- Price conversion to kopecks
- Status enum with lowercase serialization
- Internal fields hidden from JSON output

**Example:**
```rust
let product_json = r#"
{
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "MacBook Pro",
    "price": 199999,
    "category": "Electronics",
    "in_stock": true
}
"#;

let product: Product = serde_json::from_str(product_json).unwrap();
```

**Run tests:**
```bash
cargo test --bin task5
```

### Task 6: Web Server

Simple web server using Actix-web framework.

```bash
cargo run --bin task6
```

**Access:**
- URL: [http://127.0.0.1:8080](http://127.0.0.1:8080)
- Response: "Hello, world!"

**Features:**
- Single GET endpoint at root path
- Async request handler
- Actix-web runtime

## Running All Examples

Run the main program:
```bash
cargo run
```

Run specific examples:
```bash
cargo run --bin task2
cargo run --bin task3
cargo run --bin practice1
cargo run --bin practice2
cargo run --bin task4
cargo run --bin task5
cargo run --bin task6
```

## Key Concepts Demonstrated

1. **Custom Future Implementation**: Manual implementation of the Future trait with proper Pin and Poll handling
2. **Timeout Patterns**: Race between future completion and timeout timer
3. **Backpressure**: Bounded channels with timeout-based flow control
4. **Async I/O**: Non-blocking TCP networking with AsyncRead/AsyncWrite
5. **Custom Serialization**: Serde customization with validators and transformers
6. **Web Services**: HTTP server with async handlers
7. **Concurrency**: Spawning multiple async tasks and joining results
8. **Waker Protocol**: Low-level async runtime interaction

## Testing

Run all tests:
```bash
cargo test
```

Run tests for specific modules:
```bash
cargo test --bin task5
```

## Learning Path

Recommended order for understanding the concepts:

1. Start with [task3.rs](src/bin/task3.rs) - Basic Future implementation
2. Move to [task2.rs](src/bin/task2.rs) - Understand wakers
3. Try [practice1.rs](src/bin/practice1.rs) - Timeout pattern
4. Explore [practice2.rs](src/bin/practice2.rs) - Concurrency control
5. Study [main.rs](src/main.rs) - Spawning tasks
6. Check [task4.rs](src/bin/task4.rs) - Async I/O
7. Examine [task5.rs](src/bin/task5.rs) - Advanced serialization
8. Run [task6.rs](src/bin/task6.rs) - Web server

## License

Educational project - use freely for learning purposes.
