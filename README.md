# Spaceship Language Specification

Spaceship is a high-performance systems automation language designed to replace legacy shell scripting. It features a strict, Rust-inspired syntax, a powerful fixed-width type system, and a novel JIT (Just-In-Time) compilation model for POSIX commands, all built on top of LLVM.

## Core Principles

- **Performance:** Statically typed and JIT-compiled for maximum execution speed.
- **Security:** Eliminates shell injection vulnerabilities through a strict `Process` API powered by direct OS-level syscalls.
- **Reliability:** A clear, explicit error handling model based on POSIX exit codes.
- **Modern Syntax:** A clean, Rust-inspired syntax that is easy to read and write.

## Type System

Spaceship uses a strict, fixed-width type system. There is no type inference; all types must be explicitly declared.

### Type Table

| Type             | Description                                       | Syntax Example                                  |
|------------------|---------------------------------------------------|-------------------------------------------------|
| `bool`           | Boolean type                                      | `let is_active: bool = true;`                   |
| `i8` - `i128`    | Fixed-width signed integers                       | `let user_id: i64;`                             |
| `f32`, `f64`     | Floating-point numbers                            | `const PI: f64 = 3.14159;`                      |
| `[u8]`           | Raw byte array (string)                           | `let message: [u8] = "hello";`                  |
| `[<type>; <size>]` | Fixed-size array                               | `let buffer: [i8; 1024];`                       |
| `HashMap<K, V>`  | Hash map                                          | `let scores: HashMap<[u8], i32>;`               |

## Error Handling: The `!i32` Contract

Spaceship uses an explicit error handling mechanism that maps directly to POSIX exit codes and `errno`. Any function that can fail must declare its return type with a `!` prefix, indicating that it returns an error contract.

**Example:** `fn readFile(path: [u8]) -> !i32`

- On success, the function returns a non-zero value.
- On failure, it returns a standard POSIX error code (e.g., `ENOENT` for "No such file or directory").

This contract is enforced by the `check {} except {}` block.

```rust
check {
    // Code that might fail
    let fd: i32 = readFile("my_file.txt");
} except {
    // This block executes if readFile() returns an error code
    // The error code is implicitly available in the `err` variable
    Posix.write(stdout, "Error reading file: " + err);
}
```

## Execution and Pipeline Model

### Secure Process Execution with the `Syscalls` Library

All external commands are executed using the `Process` API. This API is powered by a backend `Syscalls` runtime library that interfaces directly with the host operating system's process creation APIs (e.g., `fork`/`execve` on Linux/macOS, `CreateProcess` on Windows).

This is a critical security feature that prevents shell injection vulnerabilities by design, as arguments are passed as a structured array, not a raw string to be parsed by a shell.

**Example:** `Process("ls", ["-l", "/home/user"])`

### Deferred Execution

Spaceship uses a deferred execution model for command pipelines, inspired by fluent APIs. Operations are chained together using `.then()`, but no execution occurs until `.run()` is called.

**Example:**
```rust
let pipeline = Process("grep", ["-r", "keyword", "."])
    .then(Process("wc", ["-l"]));

// Nothing has executed yet.

let result = pipeline.run(); // The pipeline is now executed.
```

### The `jit!` Directive: Shell-to-Native Translation

The `jit!` directive is a powerful compiler feature that translates shell scripts into native POSIX logic and JIT-compiles them directly into the LLVM execution path. This provides a significant performance and security advantage over traditional shell script execution.

**Example:** `jit!("deploy.sh");`

```rust
jit!("deploy.sh");
```

This allows developers to leverage existing shell scripts while benefiting from the performance and security of the Spaceship runtime.

## Standard Library

The primary standard library for Spaceship is the JIT-compiled POSIX layer, along with the `Syscalls` runtime library. This ensures that all file I/O, process management, and other system-level operations are as fast and efficient as possible.

## Usage Examples

### Data Structures

```rust
// Declare a fixed-size array of 4 64-bit integers
let vector: [i64; 4];

// Declare a map with string keys and 32-bit integer values
let user_ages: HashMap<[u8], i32>;

// Accessing an element (syntax)
vector[2] = 100;
user_ages["jules"] = 32;
```

### Function Definition and Error Handling

```rust
// Definition for a function that can fail
fn open_or_fail(path: [u8]) -> !i32 {
    // ... low-level POSIX call to open the file ...
    // Returns a file descriptor (i32) on success or an error code on failure.
}

fn main() {
    check {
        let file_descriptor: i32 = open_or_fail("/etc/hosts");
    } except {
        // The 'err' variable is implicitly available and holds the i32 error code.
        Posix.write(stdout, "Failed to open file with error code: " + err);
    }
}
```

### Process Pipelines

```rust
// Find all .log files, count their lines, sort numerically, and get the top 5.
let pipeline = Process("find", [".", "-name", "*.log"])
    .then(Process("xargs", ["wc", "-l"]))
    .then(Process("sort", ["-n"]))
    .then(Process("tail", ["-n", "5"]));

// Execute the entire pipeline.
let top_five_logs: [u8] = pipeline.run();

Posix.write(stdout, top_five_logs);
```

## Performance & Benchmarks

The primary goal of Spaceship is to provide a significant performance improvement over traditional, interpreted shell scripting languages like Bash. By using a JIT-compiler with LLVM, we aim to execute common systems administration and automation tasks an order of magnitude faster.

### Hypothetical Benchmark

The following benchmark is **hypothetical** and serves to illustrate the performance goals of the project. The task is to count the total number of lines in all `.log` files within a large directory structure.

| Language / Method        | Time (seconds) | Performance Multiplier | Notes                                           |
|--------------------------|----------------|------------------------|-------------------------------------------------|
| `bash` (find + xargs + wc) | ~12.5s         | 1x (Baseline)          | High process creation overhead.                 |
| `python` (os.walk)         | ~7.8s          | ~1.6x                  | Faster, but still interpreted.                  |
| **`Spaceship`** (Goal)     | **~0.9s**      | **~14x**               | JIT-compiled native code with minimal overhead. |

This theoretical benchmark highlights the advantages of avoiding interpreter overhead and leveraging direct, compiled POSIX calls for process management and I/O.
Note : The *standard library* is *still under development* .
