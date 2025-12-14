#ifndef ORBIT_LEXER_H
#define ORBIT_LEXER_H

#include <string>
#include <vector>

namespace orbit {

enum Token {
  // End of file
  TOK_EOF = -1,

  // Commands and identifiers
  TOK_IDENTIFIER = -2,
  TOK_LITERAL_STRING = -3,
  TOK_LITERAL_INTEGER = -4,

  // Keywords
  TOK_IF = -5,
  TOK_FOR = -6,
  TOK_CHECK = -7,
  TOK_EXCEPT = -8,

  // Operators and symbols
  TOK_LPAREN = '(',
  TOK_RPAREN = ')',
  TOK_LBRACE = '{',
  TOK_RBRACE = '}',
  TOK_DOT = '.',
  TOK_COMMA = ',',
  TOK_COLON = ':',
  TOK_LBRACKET = '[',
  TOK_RBRACKET = ']',

  // Method chaining operator
  TOK_THEN = -9,
};

class Lexer {
public:
  Lexer(const std::string& source);
  std::vector<std::pair<Token, std::string>> tokenize();

private:
  std::string m_source;
  size_t m_cursor;
};

}

#endif // ORBIT_LEXER_H
