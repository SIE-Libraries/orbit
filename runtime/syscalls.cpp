#include "syscalls.h"
#include <iostream>

#if defined(_WIN32) || defined(_WIN64)
    #include <windows.h>
    #include <processthreadsapi.h>
#elif defined(__linux__) || (defined(__APPLE__) && defined(__MACH__))
    #include <unistd.h>
    #include <sys/wait.h>
#endif

extern "C" {

int spaceship_run_process(const char* command, char* const* args) {
#if defined(_WIN32) || defined(_WIN64)
    // Windows implementation using CreateProcess()
    std::cerr << "Syscalls: Windows process execution not yet implemented." << std::endl;
    return -1;

#elif defined(__linux__) || (defined(__APPLE__) && defined(__MACH__))
    // Linux and macOS implementation using fork() and execve()
    pid_t pid = fork();

    if (pid == -1) {
        // Fork failed
        perror("fork");
        return -1;
    } else if (pid == 0) {
        // Child process
        execve(command, args, nullptr);
        // If execve returns, an error occurred
        perror("execve");
        exit(EXIT_FAILURE);
    } else {
        // Parent process
        int status;
        if (waitpid(pid, &status, 0) == -1) {
            perror("waitpid");
            return -1;
        }
        if (WIFEXITED(status)) {
            return WEXITSTATUS(status);
        }
        return -1; // Indicate an abnormal termination
    }
#else
    #error "Unsupported platform: Spaceship Syscalls requires Windows, Linux, or macOS."
    return -1;
#endif
}

} // extern "C"
