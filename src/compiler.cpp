#include "compiler.h"
#include "llvm/IR/Verifier.h"
#include <vector>

namespace orbit {

Compiler::Compiler() {
  m_context = std::make_unique<llvm::LLVMContext>();
  m_module = std::make_unique<llvm::Module>("OrbitShellModule", *m_context);
  m_builder = std::make_unique<llvm::IRBuilder<>>(*m_context);
}

void Compiler::compile(ASTNode* node) {
  if (node) {
    node->CodeGen(*this);
  }
  // For debugging purposes, print the generated IR.
  m_module->print(llvm::errs(), nullptr);
}

// Placeholder implementations for CodeGen methods.
// The actual logic will be filled in as the compiler develops.

llvm::Value *NumberNode::CodeGen(Compiler& compiler) {
  // TODO: Implement number literal code generation.
  return nullptr;
}

llvm::Value *StringNode::CodeGen(Compiler& compiler) {
  // TODO: Implement string literal code generation.
  return nullptr;
}

llvm::Value *MethodCallNode::CodeGen(Compiler& compiler) {
  // Example implementation for a simple native call like Log.info("message").
  // This is a placeholder to demonstrate calling an external C function.

  // Look up the function in the module's symbol table.
  // For this example, we'll hardcode the function name.
  // A real implementation would involve a more robust function lookup mechanism.
  if (Callee == "info") {
    llvm::Function *calleeF = compiler.getModule().getFunction("_orbit_log_info");
    if (!calleeF) {
      // If the function is not yet declared, create a prototype.
      // This assumes a signature: void _orbit_log_info(char*).
      std::vector<llvm::Type*> argTypes(1, llvm::Type::getInt8PtrTy(compiler.getContext()));
      llvm::FunctionType *funcType = llvm::FunctionType::get(
          llvm::Type::getVoidTy(compiler.getContext()), argTypes, false);

      calleeF = llvm::Function::Create(
          funcType, llvm::Function::ExternalLinkage, "_orbit_log_info", &compiler.getModule());
    }

    // Generate code for the arguments.
    std::vector<llvm::Value*> argsV;
    for (const auto &arg : Args) {
        argsV.push_back(arg->CodeGen(compiler));
    }

    // Create the call instruction.
    return compiler.getBuilder().CreateCall(calleeF, argsV, "logcall");
  }

  // Return null if the method is not recognized.
  return nullptr;
}

llvm::Value *ChainNode::CodeGen(Compiler& compiler) {
  // TODO: Implement .then chain code generation.
  // This will involve generating code for the Lhs, then piping its
  // result into the code generated for the Rhs.
  return nullptr;
}

llvm::Value *CheckExceptNode::CodeGen(Compiler& compiler) {
  // TODO: Implement check/except error handling code generation.
  // This will likely involve setting up basic blocks for the 'check'
  // and 'except' clauses and handling exceptions or error states.
  return nullptr;
}

llvm::Value *ProcessCallNode::CodeGen(Compiler& compiler) {
  // TODO: Implement external process call code generation.
  // This will generate a call to a runtime function that handles
  // executing external commands.
  return nullptr;
}

}
