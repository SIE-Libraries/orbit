#ifndef SPACESHIP_COMPILER_H
#define SPACESHIP_COMPILER_H

#include "ast.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include <memory>
#include <map>

namespace spaceship {

class Compiler {
public:
  Compiler();
  void compile(ASTNode* node);

  // Getters for LLVM components
  llvm::LLVMContext& getContext() { return *m_context; }
  llvm::Module& getModule() { return *m_module; }
  llvm::IRBuilder<>& getBuilder() { return *m_builder; }

  // Symbol table for variable lookups
  std::map<std::string, llvm::Value*> NamedValues;

private:
  std::unique_ptr<llvm::LLVMContext> m_context;
  std::unique_ptr<llvm::Module> m_module;
  std::unique_ptr<llvm::IRBuilder<>> m_builder;
};

} // namespace spaceship

#endif // SPACESHIP_COMPILER_H
