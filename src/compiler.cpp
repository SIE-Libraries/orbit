#include "compiler.h"
#include "llvm/IR/Verifier.h"

namespace spaceship {

Compiler::Compiler() {
  m_context = std::make_unique<llvm::LLVMContext>();
  m_module = std::make_unique<llvm::Module>("SpaceshipJIT", *m_context);
  m_builder = std::make_unique<llvm::IRBuilder<>>(*m_context);
}

void Compiler::compile(ASTNode* node) {
  if (node) {
    node->CodeGen(*this);
  }
  // For debugging purposes, print the generated IR.
  m_module->print(llvm::errs(), nullptr);
}

//--- CodeGen Implementations ---

llvm::Value* TypeNode::CodeGen(Compiler& compiler) {
  // TODO: Convert the Spaceship type name to a corresponding LLVM type.
  // This will involve parsing the type name (e.g., "i23") and creating
  // an llvm::IntegerType with the correct bit-width.
  // For u8[], this would return an array or pointer type.
  return nullptr;
}

llvm::Value* IntegerLiteralNode::CodeGen(Compiler& compiler) {
  // TODO: Create an LLVM constant integer. The type will need to be
  // determined from context or a type suffix in a fuller implementation.
  // For now, we can default to i64 for simplicity.
  return llvm::ConstantInt::get(llvm::Type::getInt64Ty(compiler.getContext()), Val, true);
}

llvm::Value* StringLiteralNode::CodeGen(Compiler& compiler) {
  // TODO: Create a global string pointer for the u8[] literal.
  // This is the standard way to handle string literals in LLVM.
  return compiler.getBuilder().CreateGlobalStringPtr(Val, "str_literal");
}

llvm::Value* VarDeclNode::CodeGen(Compiler& compiler) {
  // TODO: Implement variable declaration.
  // 1. Get the LLVM type from VarType->CodeGen().
  // 2. Create an alloca instruction to allocate stack space.
  // 3. If InitialValue is present, generate its code and store the result.
  // 4. Add the variable to the compiler's symbol table (NamedValues).
  return nullptr;
}

llvm::Value* FnDeclNode::CodeGen(Compiler& compiler) {
  // TODO: Implement function declaration.
  // 1. Create the function prototype (mangling names might be necessary).
  // 2. Create the entry basic block.
  // 3. Set up arguments in the symbol table.
  // 4. Generate code for each statement in the Body.
  // 5. Handle the return, especially the !i32 error contract.
  // 6. Verify the generated function.
  return nullptr;
}

llvm::Value* ProcessCallNode::CodeGen(Compiler& compiler) {
  // TODO: Implement secure process call.
  // This will generate a call to a unified backend function (e.g., a C function
  // linked into the JIT) that takes the command and arguments and uses execve.
  // This is a critical security feature.
  return nullptr;
}

llvm::Value* PipelineNode::CodeGen(Compiler& compiler) {
  // TODO: Implement deferred execution pipeline.
  // The code generation for Lhs and Rhs should be connected in a way
  // that the output of the first becomes the input of the second.
  // The final execution is triggered by a .run() call, which this
  // skeleton does not yet include.
  return nullptr;
}

llvm::Value* JitDirectiveNode::CodeGen(Compiler& compiler) {
  // TODO: Implement the @jit shell-to-native translation engine.
  // 1. Read the content of the shell script at FilePath.
  // 2. Parse the shell script into a sequence of POSIX-equivalent commands.
  // 3. Translate these commands into LLVM IR, similar to ProcessCallNode.
  // 4. Inline the generated IR into the current function.
  // This is a major and complex part of the compiler.
  return nullptr;
}

llvm::Value* CheckExceptNode::CodeGen(Compiler& compiler) {
  // TODO: Implement check/except error handling.
  // This will involve setting up basic blocks for the 'check' and 'except'
  // clauses. The code will need to check the return value of functions
  // with the !i32 contract and jump to the 'except' block on failure.
  return nullptr;
}

} // namespace spaceship
