# Aes (Asynchronous Event System)

A thread-safe, asynchronous event system for Rust. Aes allows developers to create **events that can trigger multiple callbacks concurrently**, making it easier to build event-driven code.

## Features

* **Concurrent execution of callbacks**
  Every connected callback runs in its own Tokio task, allowing high-performance, parallel execution.

* **Simple API**
  Provides intuitive methods to connect, disconnect, and fire events, keeping your code clean and maintainable.

> **Note:** This library requires a Tokio runtime to function. Make sure your `main` function uses `#[tokio::main]` or you create a runtime manually.
> **Caution:** This project is developed by a beginner in Rust. 


