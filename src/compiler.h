#ifndef ORBIT_COMPILER_H
#define ORBIT_COMPILER_H

#include "ast.h"
#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"
#include "llvm/IR/Module.h"
#include <memory>

namespace orbit {

class Compiler {
public:
  Compiler();
  void compile(ASTNode* node);

public:
  llvm::LLVMContext& getContext() { return *m_context; }
  llvm::Module& getModule() { return *m_module; }
  llvm::IRBuilder<>& getBuilder() { return *m_builder; }

private:
  std::unique_ptr<llvm::LLVMContext> m_context;
  std::unique_ptr<llvm::Module> m_module;
  std::unique_ptr<llvm::IRBuilder<>> m_builder;
};

}

#endif // ORBIT_COMPILER_H
