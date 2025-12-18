#ifndef SPACESHIP_LEXER_H
#define SPACESHIP_LEXER_H

#include <string>
#include <vector>

namespace spaceship {

enum Token {
  // Meta
  TOK_EOF = -1,
  TOK_UNKNOWN = -2,

  // Literals
  TOK_IDENTIFIER = -3,
  TOK_LITERAL_INTEGER = -4,
  TOK_LITERAL_STRING = -5, // Represents u8[]

  // Keywords
  TOK_VAR = -6,   // var
  TOK_FN = -7,    // fn
  TOK_CONST = -8, // const
  TOK_CHECK = -9, // check
  TOK_EXCEPT = -10, // except
  TOK_MAP = -11,    // map

  // Types
  TOK_TYPE_I1 = -20,
  TOK_TYPE_I8 = -21,
  TOK_TYPE_I16 = -22,
  TOK_TYPE_I32 = -23,
  TOK_TYPE_I64 = -24,
  TOK_TYPE_I128 = -25,
  TOK_TYPE_F32 = -26,
  TOK_TYPE_F64 = -27,
  TOK_TYPE_U8_ARRAY = -28, // u8[]

  // Directives
  TOK_AT_JIT = -30, // @jit

  // Operators & Punctuation
  TOK_LPAREN = '(',
  TOK_RPAREN = ')',
  TOK_LBRACE = '{',
  TOK_RBRACE = '}',
  TOK_LBRACKET = '[',
  TOK_RBRACKET = ']',
  TOK_COMMA = ',',
  TOK_DOT = '.',
  TOK_BANG = '!', // For !i32 error contract
};

// Represents a single token produced by the lexer
struct TokenInfo {
  Token type;
  std::string value;
  int line;
  int col;
};

class Lexer {
public:
  Lexer(const std::string& source);
  std::vector<TokenInfo> tokenize();

private:
  std::string m_source;
  size_t m_cursor;
  int m_line;
  int m_col_start;
};

} // namespace spaceship

#endif // SPACESHIP_LEXER_H
