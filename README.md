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
