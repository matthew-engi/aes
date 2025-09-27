# Avent (Asynchronous Event System)

A thread-safe, asynchronous event system for Rust. This library allows you to register multiple callbacks, fire events concurrently, and handle synchronous callbacks safely.

## Features
- Thread-safe and `Send + Sync` compatible.
- Concurrent execution of all callbacks using Tokio tasks.
