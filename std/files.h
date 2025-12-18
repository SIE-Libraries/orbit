#ifndef ORBIT_STD_FILES_H
#define ORBIT_STD_FILES_H

#include <string>

// Forward declaration for the base object type in Orbit.
// In a real implementation, this would be a more complex class.
class OrbitObject {};

// C++ signatures for the core file I/O functions that Orbit will expose.
// These functions will be implemented in the runtime and their names will be
// used by the compiler to generate the corresponding LLVM IR.
// Note: Using std::string in an extern "C" block is generally not recommended
// due to C++ name mangling. A pure C interface with const char* would be more
// portable. However, we are following the specified interface.

extern "C" {
  /**
   * @brief Opens a file at the given path.
   *
   * @param path The path to the file.
   * @return A pointer to an OrbitObject representing the file.
   */
  OrbitObject* Files_open(const std::string& path);

  /**
   * @brief Checks if a file exists at the given path.
   *
   * @param path The path to the file.
   * @return True if the file exists, false otherwise.
   */
  bool Files_exists(const std::string& path);
}

#endif // ORBIT_STD_FILES_H
