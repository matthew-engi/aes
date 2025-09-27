<div align="center">
  <img src="https://github.com/user-attachments/assets/61fabea7-f4d6-446a-a54b-07a617465a27"/>
</div>

<h1 align="center">Ewait - Asynchronous Event System</h1>
<p align="center">
  A thread-safe, asynchronous event system for Rust.
</p>

Ewait allows developers to create **events that can trigger multiple callbacks concurrently**, making it easier to build event-driven code.

---

## Features

* **Concurrent execution of callbacks**
  Every connected callback runs in its own Tokio task, allowing high-performance, parallel execution.

* **Simple API**
  Provides intuitive methods to connect, disconnect, and fire events, keeping your code clean and maintainable.

> [!WARNING]
> This library requires a Tokio runtime to function. Make sure your `main` function uses `#[tokio::main]` or you create a runtime manually.

> [!CAUTION]
> This project is developed by a beginner in Rust. 


