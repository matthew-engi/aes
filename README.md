<div align="center">
  <img src="https://github.com/user-attachments/assets/46f39d15-7143-4791-b0ac-4887deab1227"/>
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
* Conveniently provides a `JoinHandle<R>` to `.await` results to if the results are of interest.
* Automatically manages the distribution of Arc references between threads.

## Installation

Simply run `cargo add ewait` in the desired Rust project.
Here is an example on how to use the Event struct:
```rust
use tokio;
use ewait::Event;
use std::sync::Arc;

async fn hello(_: Arc<()>) {
    println!("Hello World!")
}

#[tokio::main]
async fn main() {
    let mut event1: Event<()> = Event::new();
    event1.connect(hello);
    event1.fire(());
}
```

---

> [!WARNING]
> This library requires a Tokio runtime to function. Make sure your `main` function uses `#[tokio::main]`.

> [!CAUTION]
> This project is developed by a new Rustacean. The testing is limited.


