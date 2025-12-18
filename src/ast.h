#ifndef SPACESHIP_AST_H
#define SPACESHIP_AST_H

#include <string>
#include <vector>
#include <memory>

// Forward declaration for LLVM Value class.
namespace llvm {
  class Value;
}

namespace spaceship {

// Forward-declaration of the Compiler class to avoid circular dependencies.
class Compiler;

/// ASTNode - Base class for all AST nodes.
class ASTNode {
public:
  virtual ~ASTNode() = default;
  virtual llvm::Value *CodeGen(Compiler& compiler) = 0;
};

/// ExpressionNode - Base class for nodes that represent a value.
class ExpressionNode : public ASTNode {};

/// StatementNode - Base class for nodes that represent an action or declaration.
class StatementNode : public ASTNode {};

/// TypeNode - Represents a type in the language, e.g., i32, f64, u8[].
class TypeNode : public ASTNode {
protected:
  std::string TypeName;
  // In a full implementation, this would store detailed type info,
  // such as bit-width for arbitrary precision integers.
public:
  TypeNode(const std::string& typeName) : TypeName(typeName) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
  const std::string& getTypeName() const { return TypeName; }
};

/// ArrayTypeNode - Represents an array type, e.g., [8]i32.
class ArrayTypeNode : public TypeNode {
  int Size;
  std::unique_ptr<TypeNode> ElementType;

public:
  ArrayTypeNode(int size, std::unique_ptr<TypeNode> elementType)
      : TypeNode("array"), Size(size), ElementType(std::move(elementType)) {}
  llvm::Value* CodeGen(Compiler& compiler) override;
};

/// MapTypeNode - Represents a map type, e.g., map[u8[]]i32.
class MapTypeNode : public TypeNode {
  std::unique_ptr<TypeNode> KeyType;
  std::unique_ptr<TypeNode> ValueType;

public:
  MapTypeNode(std::unique_ptr<TypeNode> keyType, std::unique_ptr<TypeNode> valueType)
      : TypeNode("map"), KeyType(std::move(keyType)), ValueType(std::move(valueType)) {}
  llvm::Value* CodeGen(Compiler& compiler) override;
};

/// IndexAccessNode - Expression for accessing an array or map element, e.g., my_array[i].
class IndexAccessNode : public ExpressionNode {
  std::unique_ptr<ExpressionNode> Aggregate;
  std::unique_ptr<ExpressionNode> Index;

public:
  IndexAccessNode(std::unique_ptr<ExpressionNode> aggregate, std::unique_ptr<ExpressionNode> index)
      : Aggregate(std::move(aggregate)), Index(std::move(index)) {}
  llvm::Value* CodeGen(Compiler& compiler) override;
};

/// IntegerLiteralNode - Expression for numeric literals.
class IntegerLiteralNode : public ExpressionNode {
  long long Val; // Use long long to accommodate larger integers like i64
public:
  IntegerLiteralNode(long long val) : Val(val) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// StringLiteralNode - Expression for string literals (u8[]).
class StringLiteralNode : public ExpressionNode {
  std::string Val;
public:
  StringLiteralNode(const std::string& val) : Val(val) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// VarDeclNode - Statement for a variable declaration, e.g., var x i32.
class VarDeclNode : public StatementNode {
  std::string VarName;
  std::unique_ptr<TypeNode> VarType;
  std::unique_ptr<ExpressionNode> InitialValue; // Optional initial value

public:
  VarDeclNode(const std::string& varName, std::unique_ptr<TypeNode> varType, std::unique_ptr<ExpressionNode> initialValue = nullptr)
      : VarName(varName), VarType(std::move(varType)), InitialValue(std::move(initialValue)) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// FnDeclNode - Statement for a function declaration, e.g., fn main() !i32.
class FnDeclNode : public StatementNode {
  std::string FnName;
  std::vector<std::unique_ptr<VarDeclNode>> Args;
  std::unique_ptr<TypeNode> ReturnType; // Can be null for void
  bool IsErrorContract; // True if the return type is an error contract, e.g., !i32
  std::vector<std::unique_ptr<StatementNode>> Body;

public:
  FnDeclNode(const std::string& fnName, std::vector<std::unique_ptr<VarDeclNode>> args,
             std::unique_ptr<TypeNode> returnType, bool isErrorContract,
             std::vector<std::unique_ptr<StatementNode>> body)
      : FnName(fnName), Args(std::move(args)), ReturnType(std::move(returnType)),
        IsErrorContract(isErrorContract), Body(std::move(body)) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// ProcessCallNode - Represents a secure external command execution.
class ProcessCallNode : public ExpressionNode {
  std::string Command;
  std::vector<std::unique_ptr<ExpressionNode>> Args;

public:
  ProcessCallNode(const std::string &command,
                  std::vector<std::unique_ptr<ExpressionNode>> args)
      : Command(command), Args(std::move(args)) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// PipelineNode - Represents the .then() deferred execution pipeline.
class PipelineNode : public ExpressionNode {
  std::unique_ptr<ExpressionNode> Lhs;
  std::unique_ptr<ExpressionNode> Rhs;

public:
  PipelineNode(std::unique_ptr<ExpressionNode> lhs, std::unique_ptr<ExpressionNode> rhs)
      : Lhs(std::move(lhs)), Rhs(std::move(rhs)) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// JitDirectiveNode - Represents the @jit("file.sh") directive.
class JitDirectiveNode : public StatementNode {
    std::string FilePath;
public:
    JitDirectiveNode(const std::string& filePath) : FilePath(filePath) {}
    llvm::Value *CodeGen(Compiler& compiler) override;
};

/// CheckExceptNode - Represents a check {} except {} error handling block.
class CheckExceptNode : public StatementNode {
  std::vector<std::unique_ptr<StatementNode>> CheckBlock;
  std::vector<std::unique_ptr<StatementNode>> ExceptBlock;

public:
  CheckExceptNode(std::vector<std::unique_ptr<StatementNode>> checkBlock,
                  std::vector<std::unique_ptr<StatementNode>> exceptBlock)
      : CheckBlock(std::move(checkBlock)), ExceptBlock(std::move(exceptBlock)) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

} // namespace spaceship

#endif // SPACESHIP_AST_H
