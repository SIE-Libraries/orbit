#ifndef SPACESHIP_SYSCALLS_H
#define SPACESHIP_SYSCALLS_H

#include <vector>
#include <string>

// The C-linkage function that will be called from the JIT-compiled code.
// It takes a command and an array of arguments, and returns the exit code.
extern "C" {
    int spaceship_run_process(const char* command, char* const* args);
}

#endif // SPACESHIP_SYSCALLS_H
