🚀 Orbit Shell & Spaceship Syntax
A Modern, Compiled System Scripting Language for the 21st Century
Orbit is a new, high-performance shell and system scripting environment powered by the LLVM compiler infrastructure. Designed to replace traditional, fragile POSIX shell scripts, Orbit introduces the Spaceship syntax—a clean, object-oriented language focused on reliability, speed, and structured data handling.

✨ Why Use Orbit and Spaceship?
⚡️ Performance & Compilation: Scripts are compiled via LLVM, delivering execution speeds comparable to compiled binaries, vastly outperforming interpreted shells like Bash or Python-based automation.

🏗️ Modern, Object-Oriented Syntax: The Spaceship syntax treats every command and system utility as a first-class object, supporting method chaining and explicit data flow, similar to modern programming languages.

Pipelining: Replaces the cryptic pipe (|) with clear method chaining: Process(cmd1, [args]).then(cmd2, [args]).

Explicit Standard Library: System tasks like file access and JSON parsing are handled natively: Files.open() and Json.reader().

🛡️ Robust Error Handling: Eliminates fragile exit code checks by introducing typed exceptions via the familiar check { ... } except (e: ErrorObject) { ... } block structure.

📦 Native Tool Encapsulation: Complex external tools are wrapped in native, type-safe modules, providing intuitive control over external processes:

Git Module: Git.pull().commit("Fix bug")

Docker Module: Docker.build("app:latest")

🚫 Non-POSIX Freedom: Orbit is not constrained by POSIX compatibility, allowing us to build a streamlined, safe, and internally consistent environment from the ground up.🚀 Orbit Shell & Spaceship Syntax
A Modern, Compiled System Scripting Language for the 21st Century
Orbit is a new, high-performance shell and system scripting environment powered by the LLVM compiler infrastructure. Designed to replace traditional, fragile POSIX shell scripts, Orbit introduces the Spaceship syntax—a clean, object-oriented language focused on reliability, speed, and structured data handling.

✨ Why Use Orbit and Spaceship?
⚡️ Performance & Compilation: Scripts are compiled via LLVM, delivering execution speeds comparable to compiled binaries, vastly outperforming interpreted shells like Bash or Python-based automation.

🏗️ Modern, Object-Oriented Syntax: The Spaceship syntax treats every command and system utility as a first-class object, supporting method chaining and explicit data flow, similar to modern programming languages.

Pipelining: Replaces the cryptic pipe (|) with clear method chaining: Process(cmd1, [args]).then(cmd2, [args]).

Explicit Standard Library: System tasks like file access and JSON parsing are handled natively: Files.open() and Json.reader().

🛡️ Robust Error Handling: Eliminates fragile exit code checks by introducing typed exceptions via the familiar check { ... } except (e: ErrorObject) { ... } block structure.

📦 Native Tool Encapsulation: Complex external tools are wrapped in native, type-safe modules, providing intuitive control over external processes:

Git Module: Git.pull().commit("Fix bug")

Docker Module: Docker.build("app:latest")

🚫 Non-POSIX Freedom: Orbit is not constrained by POSIX compatibility, allowing us to build a streamlined, safe, and internally consistent environment from the ground up.

💻 A Simple Spaceship Example

'''Orbit
check {
    # Clone repository and switch to a feature branch using the native Git module
    Git.clone("repo_url", "local_dir").checkout("feature/build-update")

    # Build the Docker image, piping the result to a custom build function
    Docker.build("app:latest").then(Deploy.validate)

    # Use native collections and control flow
    for (i: range(10)) {
        Log.info("Running iteration #" + i.toString())
    }
} except (e: GitError) {
    Log.error("Git failed: " + e.message)
    Process("notify-user", ["Build failed due to Git issues"])
}
'''
