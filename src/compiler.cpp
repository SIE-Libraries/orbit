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
    // Note: This function doesn't return a Value, but it's part of the ASTNode interface.
    // In a more complex compiler, we might have a separate AST hierarchy for types.
    // For now, we will return null and the caller will use a separate method to get the type.
    // A better approach would be to have a `getType()` method on TypeNode that returns an llvm::Type*.

    // For the purpose of this implementation, we will imagine the caller
    // will use a helper function that inspects the TypeNode. The logic for
    // converting the type name to an LLVM type is what's important here.
    return nullptr;
}

// Helper function to get the LLVM type from a TypeNode.
// This is where the core logic of the Type System Codegen resides.
llvm::Type* getLLVMType(TypeNode& typeNode, llvm::LLVMContext& context) {
    if (auto* arrayTypeNode = dynamic_cast<ArrayTypeNode*>(&typeNode)) {
        llvm::Type* elementType = getLLVMType(*arrayTypeNode->ElementType, context);
        return llvm::ArrayType::get(elementType, arrayTypeNode->Size);
    }

    const std::string& typeName = typeNode.getTypeName();

    if (typeName == "i1") return llvm::Type::getInt1Ty(context);
    if (typeName == "i8") return llvm::Type::getInt8Ty(context);
    if (typeName == "i16") return llvm::Type::getInt16Ty(context);
    if (typeName == "i32") return llvm::Type::getInt32Ty(context);
    if (typeName == "i64") return llvm::Type::getInt64Ty(context);
    if (typeName == "i128") return llvm::Type::getInt128Ty(context);
    if (typeName == "f32") return llvm::Type::getFloatTy(context);
    if (typeName == "f64") return llvm::Type::getDoubleTy(context);

    // Handle u8[] as a pointer to an 8-bit integer (char*).
    if (typeName == "u8[]") {
        return llvm::Type::getInt8PtrTy(context);
    }

    // Handle arbitrary-width integers, e.g., i23
    if (typeName.rfind("i", 0) == 0) {
        try {
            int bitWidth = std::stoi(typeName.substr(1));
            return llvm::Type::getIntNTy(context, bitWidth);
        } catch (...) {
            // Handle parsing error
            return nullptr;
        }
    }

    // Return null for unknown types
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

// Forward declaration of the helper function
llvm::Type* getLLVMType(TypeNode& typeNode, llvm::LLVMContext& context);

llvm::Value* VarDeclNode::CodeGen(Compiler& compiler) {
    // 1. Get the LLVM type from the TypeNode.
    llvm::Type* llvmType = getLLVMType(*VarType, compiler.getContext());
    if (!llvmType) {
        // Error handling: unknown type
        return nullptr;
    }

    // 2. Create an alloca instruction in the function's entry block.
    // This is the standard way to allocate local variables on the stack.
    llvm::Function* theFunction = compiler.getBuilder().GetInsertBlock()->getParent();
    llvm::IRBuilder<> TmpB(&theFunction->getEntryBlock(), theFunction->getEntryBlock().begin());
    llvm::AllocaInst* alloca = TmpB.CreateAlloca(llvmType, nullptr, VarName);

    // 3. If there is an initial value, generate its code and store it.
    if (InitialValue) {
        llvm::Value* initialVal = InitialValue->CodeGen(compiler);
        if (!initialVal) {
            // Error handling: invalid initial value
            return nullptr;
        }
        compiler.getBuilder().CreateStore(initialVal, alloca);
    }

    // 4. Add the variable to the compiler's symbol table for future lookups.
    compiler.NamedValues[VarName] = alloca;

    return alloca;
}

llvm::Value* FnDeclNode::CodeGen(Compiler& compiler) {
    // 1. Create the function prototype.
    std::vector<llvm::Type*> argTypes;
    for (const auto& arg : Args) {
        argTypes.push_back(getLLVMType(*arg->VarType, compiler.getContext()));
    }

    llvm::Type* returnType = ReturnType ? getLLVMType(*ReturnType, compiler.getContext()) : llvm::Type::getVoidTy(compiler.getContext());
    llvm::FunctionType* funcType = llvm::FunctionType::get(returnType, argTypes, false);
    llvm::Function* function = llvm::Function::Create(funcType, llvm::Function::ExternalLinkage, FnName, compiler.getModule());

    // 2. Create the entry basic block.
    llvm::BasicBlock* basicBlock = llvm::BasicBlock::Create(compiler.getContext(), "entry", function);
    compiler.getBuilder().SetInsertPoint(basicBlock);

    // 3. Set up arguments in the symbol table.
    compiler.NamedValues.clear();
    for (auto& arg : function->args()) {
        // Create an alloca for each argument.
        llvm::AllocaInst* alloca = compiler.getBuilder().CreateAlloca(arg.getType(), nullptr, arg.getName());
        // Store the initial value into the alloca.
        compiler.getBuilder().CreateStore(&arg, alloca);
        // Add arguments to the symbol table.
        compiler.NamedValues[arg.getName()] = alloca;
    }

    // 4. Generate code for each statement in the Body.
    for (const auto& stmt : Body) {
        stmt->CodeGen(compiler);
    }

    // 5. Handle the return. (Simplified)
    if (function->getReturnType()->isVoidTy()) {
        compiler.getBuilder().CreateRetVoid();
    } else {
        // In a real implementation, we would need a return statement in the AST.
        // For now, we'll just return a zero value.
        compiler.getBuilder().CreateRet(llvm::Constant::getNullValue(function->getReturnType()));
    }

    // 6. Verify the generated function.
    llvm::verifyFunction(*function);

    return function;
}

llvm::Value* ProcessCallNode::CodeGen(Compiler& compiler) {
    // 1. Get the function prototype for spaceship_run_process.
    llvm::Function* calleeF = compiler.getModule().getFunction("spaceship_run_process");
    if (!calleeF) {
        // Create the prototype: int spaceship_run_process(const char*, char* const*)
        llvm::Type* intType = llvm::Type::getInt32Ty(compiler.getContext());
        llvm::Type* strType = llvm::Type::getInt8PtrTy(compiler.getContext());
        llvm::Type* strArrType = llvm::Type::getInt8PtrTy(compiler.getContext())->getPointerTo();

        std::vector<llvm::Type*> argTypes = {strType, strArrType};
        llvm::FunctionType* funcType = llvm::FunctionType::get(intType, argTypes, false);

        calleeF = llvm::Function::Create(
            funcType, llvm::Function::ExternalLinkage, "spaceship_run_process", &compiler.getModule());
    }

    // 2. Prepare the arguments.
    // The first argument is the command itself.
    llvm::Value* commandStr = compiler.getBuilder().CreateGlobalStringPtr(Command, "command");

    // The second argument is a null-terminated array of C-style strings.
    std::vector<llvm::Value*> argsV;
    // The first element of the args array for execve is conventionally the program name.
    argsV.push_back(compiler.getBuilder().CreateGlobalStringPtr(Command, "arg0"));
    for (const auto& arg : Args) {
        // In a real implementation, we would need to ensure the expression
        // results in a u8[] (char*).
        argsV.push_back(arg->CodeGen(compiler));
    }
    // Add the null terminator.
    argsV.push_back(llvm::ConstantPointerNull::get(llvm::Type::getInt8PtrTy(compiler.getContext())));

    // Create the array on the stack.
    llvm::ArrayType* arrayType = llvm::ArrayType::get(llvm::Type::getInt8PtrTy(compiler.getContext()), argsV.size());
    llvm::Value* argsArray = compiler.getBuilder().CreateAlloca(arrayType, nullptr, "args_array");

    for (size_t i = 0; i < argsV.size(); ++i) {
        llvm::Value* index = llvm::ConstantInt::get(llvm::Type::getInt64Ty(compiler.getContext()), i);
        llvm::Value* ptr = compiler.getBuilder().CreateGEP(arrayType, argsArray, {llvm::ConstantInt::get(llvm::Type::getInt64Ty(compiler.getContext()), 0), index});
        compiler.getBuilder().CreateStore(argsV[i], ptr);
    }

    // Decay the array pointer to a pointer to its first element to match the function signature.
    llvm::Value* argsPtr = compiler.getBuilder().CreateBitCast(argsArray, llvm::Type::getInt8PtrTy(compiler.getContext())->getPointerTo());

    // 3. Create the call instruction.
    return compiler.getBuilder().CreateCall(calleeF, {commandStr, argsPtr});
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

llvm::Value* ArrayTypeNode::CodeGen(Compiler& compiler) {
    // This node is handled by the getLLVMType helper function.
    return nullptr;
}

llvm::Value* MapTypeNode::CodeGen(Compiler& compiler) {
    // TODO: Implement map type codegen. This will likely involve
    // creating a struct or using an external library to represent the map.
    return nullptr;
}

llvm::Value* IndexAccessNode::CodeGen(Compiler& compiler) {
    // TODO: Implement array and map access.
    // For arrays, this will generate a GetElementPtr (GEP) instruction.
    // For maps, this will generate a call to a map lookup function.
    return nullptr;
}

} // namespace spaceship
