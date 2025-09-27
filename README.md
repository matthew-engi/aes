<div align="center">
  <img src="https://github.com/user-attachments/assets/3b83f1d2-c1fd-4a80-95bd-71af86853b3f"/>
</div>
<h1 align="center">ewait - Asynchronous Event System</h1>
<p align="center">
  A thread-safe, asynchronous event system for Rust.
</p>

Ewait allows th creation of **events that can trigger multiple callbacks parallely**, making it easier to build event-driven code.

---

## Features

* Every connected callback runs in its own Tokio task, allowing parallel execution.

* Provides easy methods to connect, disconnect, and fire events, keeping your code clean.

---

> [!WARNING]
> This library requires a Tokio runtime to function. Make sure your `main` function uses `#[tokio::main]`.

> [!CAUTION]
> This project is developed by a beginner in Rust. 


