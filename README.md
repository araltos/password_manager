# Overview

As a software engineer, I am always seeking opportunities to deepen my understanding of modern programming languages and best practices. This project is a command-line password manager written in Rust, designed to let users add and view password entries for different websites.

The primary goal of this software is to explore Rust’s unique features, such as its strong type system, memory safety guarantees, and ownership model. By building a practical application, I aimed to gain hands-on experience with Rust’s syntax, user input handling, file I/O, and project structure.

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

This project was developed using the following tools:

- **Operating System:** macOS (compatible with Linux and Windows)
- **Code Editor:** Visual Studio Code
- **Version Control:** Git
- **Build Tool:** Cargo (Rust’s package manager and build system)

**Programming Language:**  
- Rust (edition 2021)

**Libraries/Crates Used:**  
- `serde` (for struct derive, future serialization)
- `serde_json` (for reading/writing JSON files)

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)
- [Serde Documentation](https://serde.rs/)

# Future Work

- Add encryption for stored passwords to enhance security
- Implement file saving/loading for persistent storage
- Use `rpassword` to hide password input
- Add support for editing and deleting existing password entries
- Improve error handling and input validation
- Create automated tests for core functionality