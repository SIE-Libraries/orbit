# Spaceship Language Specification

Spaceship is a high-performance systems automation language designed to replace legacy shell scripting. It features a strict, Go-inspired syntax, a powerful fixed-width type system, and a novel JIT (Just-In-Time) compilation model for POSIX commands, all built on top of LLVM.

## Core Principles

- **Performance:** Statically typed and JIT-compiled for maximum execution speed.
- **Security:** Eliminates shell injection vulnerabilities through a strict `Process` API.
- **Reliability:** A clear, explicit error handling model based on POSIX exit codes.
- **Modern Syntax:** A clean, Go-inspired syntax that is easy to read and write.

## Type System

Spaceship uses a strict, fixed-width type system. There is no type inference; all types must be explicitly declared.

### Type Table

| Type      | Description                                       | Notes                                                     |
|-----------|---------------------------------------------------|-----------------------------------------------------------|
| `i1`      | Boolean type                                      | Represents `true` (1) or `false` (0).                     |
| `i8`      | 8-bit signed integer                              |                                                           |
| `i16`     | 16-bit signed integer                             |                                                           |
| `i23`     | 23-bit signed integer (example)                   | Supports arbitrary bit-widths for specialized use cases.  |
| `i32`     | 32-bit signed integer                             |                                                           |
| `i64`     | 64-bit signed integer                             |                                                           |
| `i128`    | 128-bit signed integer                            |                                                           |
| `f32`     | 32-bit floating-point number                      |                                                           |
| `f64`     | 64-bit floating-point number                      |                                                           |
| `u8[]`    | Raw byte array (string)                           | Zero-copy compatible with POSIX file descriptors.         |

## Error Handling: The `!i32` Contract

Spaceship uses an explicit error handling mechanism that maps directly to POSIX exit codes and `errno`. Any function that can fail must declare its return type with a `!` prefix, indicating that it returns an error contract.

**Example:** `fn readFile(path u8[]) !i32`

- On success, the function returns a non-zero value.
- On failure, it returns a standard POSIX error code (e.g., `ENOENT` for "No such file or directory").

This contract is enforced by the `check {} except {}` block.

```go
check {
    // Code that might fail
    var fd = readFile("my_file.txt")
} except {
    // This block executes if readFile() returns an error code
    // The error code is implicitly available in the `err` variable
    Posix.write(stdout, "Error reading file: " + err)
}
```

## Execution and Pipeline Model

### Secure Process Execution

All external commands are executed using the `Process` API, which interfaces directly with `execve`. This is a critical security feature that prevents shell injection vulnerabilities by design.

**Example:** `Process("ls", ["-l", "/home/user"])`

### Deferred Execution

Spaceship uses a deferred execution model for command pipelines, inspired by fluent APIs. Operations are chained together using `.then()`, but no execution occurs until `.run()` is called.

**Example:**
```go
var pipeline = Process("grep", ["-r", "keyword", "."])
    .then(Process("wc", ["-l"]))

// Nothing has executed yet.

var result = pipeline.run() // The pipeline is now executed.
```

### The `@jit` Directive: Shell-to-Native Translation

The `@jit` directive is a powerful compiler feature that translates shell scripts into native POSIX logic and JIT-compiles them directly into the LLVM execution path. This provides a significant performance and security advantage over traditional shell script execution.

**Example:** `@jit("deploy.sh")`

When the compiler encounters this directive, it will:
1. Read and parse the `deploy.sh` script.
2. Translate the shell commands into an equivalent series of `Process` calls and POSIX operations.
3. JIT-compile the translated logic into highly optimized machine code.

This allows developers to leverage existing shell scripts while benefiting from the performance and security of the Spaceship runtime.

## Standard Library

The primary standard library for Spaceship is the JIT-compiled POSIX layer. This ensures that all file I/O, process management, and other system-level operations are as fast and efficient as possible.

## Usage Examples

### Variable and Constant Declaration

```go
// Declare a mutable 32-bit integer
var loopCounter i32 = 0

// Declare a constant byte array (string)
const GREETING u8[] = "Hello, Spaceship!"
```

### Function Definition and Error Handling

This example defines a function that attempts to open a file and returns an `!i32` error contract. It is then called within a `check/except` block.

```go
// Definition for a function that can fail
fn open_or_fail(path u8[]) !i32 {
    // ... low-level POSIX call to open the file ...
    // Returns a file descriptor (i32) on success or an error code on failure.
}

fn main() {
    check {
        // Attempt to open the file. If it fails, execution jumps to the except block.
        var file_descriptor = open_or_fail("/etc/hosts")

        // ... do something with the file_descriptor ...

    } except {
        // The 'err' variable is implicitly available and holds the i32 error code.
        Posix.write(stdout, "Failed to open file with error code: " + err)
    }
}
```

### Process Pipelines

Spaceship can construct and execute complex command pipelines with a clear, fluent syntax. Execution is deferred until the `.run()` method is called.

```go
// Find all .log files in the current directory, count the lines of each,
// sort the results numerically, and get the top 5.
// No processes are started during this setup.
var pipeline = Process("find", [".", "-name", "*.log"])
    .then(Process("xargs", ["wc", "-l"]))
    .then(Process("sort", ["-n"]))
    .then(Process("tail", ["-n", "5"]))

// Execute the entire pipeline and get the final result.
var top_five_logs = pipeline.run()

Posix.write(stdout, top_five_logs)
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
