#ifndef ORBIT_AST_H
#define ORBIT_AST_H

#include <string>
#include <vector>
#include <memory>

// Forward declaration for LLVM Value class.
// This avoids including heavy LLVM headers in the AST definition.
namespace llvm {
  class Value;
}

namespace orbit {

// Forward-declaration of the Compiler class to avoid circular dependencies.
class Compiler;

/// ASTNode - Base class for all AST nodes.
class ASTNode {
public:
  virtual ~ASTNode() = default;
  /// CodeGen - All AST nodes implement this method to generate LLVM IR.
  virtual llvm::Value *CodeGen(Compiler& compiler) = 0;
};

/// ExpressionNode - Base class for all expression nodes.
class ExpressionNode : public ASTNode {};

/// NumberNode - Expression class for numeric literals like "123".
class NumberNode : public ExpressionNode {
  int Val;
public:
  NumberNode(int Val) : Val(Val) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// StringNode - Expression class for string literals like ""abc"".
class StringNode : public ExpressionNode {
  std::string Val;
public:
  StringNode(const std::string &Val) : Val(Val) {}
  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// MethodCallNode - Represents a standard method call, e.g., Object.method(...).
class MethodCallNode : public ExpressionNode {
  std::string Callee;
  std::vector<std::unique_ptr<ExpressionNode>> Args;
  std::unique_ptr<ExpressionNode> Object;

public:
  MethodCallNode(std::unique_ptr<ExpressionNode> Object, const std::string &Callee,
                 std::vector<std::unique_ptr<ExpressionNode>> Args)
      : Object(std::move(Object)), Callee(Callee), Args(std::move(Args)) {}

  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// ChainNode - Represents the .then method chaining operator.
/// This structure explicitly separates the left and right expressions
/// to handle its unique precedence and pipeline nature.
class ChainNode : public ExpressionNode {
  std::unique_ptr<ExpressionNode> Lhs, Rhs;

public:
  ChainNode(std::unique_ptr<ExpressionNode> Lhs,
            std::unique_ptr<ExpressionNode> Rhs)
      : Lhs(std::move(Lhs)), Rhs(std::move(Rhs)) {}

  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// CheckExceptNode - Represents an error handling block.
class CheckExceptNode : public ASTNode {
  std::unique_ptr<ASTNode> CheckBlock;
  std::unique_ptr<ASTNode> ExceptBlock;

public:
  CheckExceptNode(std::unique_ptr<ASTNode> CheckBlock,
                  std::unique_ptr<ASTNode> ExceptBlock)
      : CheckBlock(std::move(CheckBlock)), ExceptBlock(std::move(ExceptBlock)) {}

  llvm::Value *CodeGen(Compiler& compiler) override;
};

/// ProcessCallNode - Represents an external command execution.
class ProcessCallNode : public ExpressionNode {
  std::string Command;
  std::vector<std::unique_ptr<ExpressionNode>> Args;

public:
  ProcessCallNode(const std::string &Command,
                  std::vector<std::unique_ptr<ExpressionNode>> Args)
      : Command(Command), Args(std::move(Args)) {}

  llvm::Value *CodeGen(Compiler& compiler) override;
};

} // namespace orbit

#endif // ORBIT_AST_H
