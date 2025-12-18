#include "lexer.h"
#include <cctype>
#include <unordered_map>

namespace spaceship {

// Mapping from keyword strings to Token enums
const std::unordered_map<std::string, Token> KEYWORDS = {
    {"var", TOK_VAR},
    {"fn", TOK_FN},
    {"const", TOK_CONST},
    {"check", TOK_CHECK},
    {"except", TOK_EXCEPT},
    {"map", TOK_MAP},
    {"i1", TOK_TYPE_I1},
    {"i8", TOK_TYPE_I8},
    {"i16", TOK_TYPE_I16},
    {"i32", TOK_TYPE_I32},
    {"i64", TOK_TYPE_I64},
    {"i128", TOK_TYPE_I128},
    {"f32", TOK_TYPE_F32},
    {"f64", TOK_TYPE_F64},
};

Lexer::Lexer(const std::string& source)
    : m_source(source), m_cursor(0), m_line(1), m_col_start(0) {}

std::vector<TokenInfo> Lexer::tokenize() {
    std::vector<TokenInfo> tokens;
    while (m_cursor < m_source.length()) {
        char currentChar = m_source[m_cursor];
        int currentCol = m_cursor - m_col_start + 1;

        if (isspace(currentChar)) {
            if (currentChar == '\n') {
                m_line++;
                m_col_start = m_cursor + 1;
            }
            m_cursor++;
            continue;
        }

        if (isalpha(currentChar)) {
            std::string identifier = "";
            while (m_cursor < m_source.length() && (isalnum(m_source[m_cursor]) || m_source[m_cursor] == '_')) {
                identifier += m_source[m_cursor];
                m_cursor++;
            }

            // Check for u8[] type
            if (identifier == "u8" && m_cursor + 1 < m_source.length() && m_source[m_cursor] == '[' && m_source[m_cursor+1] == ']') {
                tokens.push_back({TOK_TYPE_U8_ARRAY, "u8[]", m_line, currentCol});
                m_cursor += 2; // Consume []
                continue;
            }

            // Check for i<N> pattern for arbitrary-width integers
            if (identifier.rfind("i", 0) == 0 && identifier.length() > 1 && isdigit(identifier[1])) {
                tokens.push_back({TOK_TYPE_I32, identifier, m_line, currentCol}); // Using I32 as a placeholder
                continue;
            }

            auto it = KEYWORDS.find(identifier);
            if (it != KEYWORDS.end()) {
                tokens.push_back({it->second, identifier, m_line, currentCol});
            } else {
                tokens.push_back({TOK_IDENTIFIER, identifier, m_line, currentCol});
            }
            continue;
        }

        if (isdigit(currentChar)) {
            std::string number = "";
            while (m_cursor < m_source.length() && isdigit(m_source[m_cursor])) {
                number += m_source[m_cursor];
                m_cursor++;
            }
            tokens.push_back({TOK_LITERAL_INTEGER, number, m_line, currentCol});
            continue;
        }

        if (currentChar == '"') {
            m_cursor++; // Skip opening quote
            std::string str = "";
            while (m_cursor < m_source.length() && m_source[m_cursor] != '"') {
                str += m_source[m_cursor];
                m_cursor++;
            }
            m_cursor++; // Skip closing quote
            tokens.push_back({TOK_LITERAL_STRING, str, m_line, currentCol});
            continue;
        }

        if (currentChar == '@') {
            if (m_cursor + 3 < m_source.length() && m_source.substr(m_cursor + 1, 3) == "jit") {
                tokens.push_back({TOK_AT_JIT, "@jit", m_line, currentCol});
                m_cursor += 4;
                continue;
            }
        }

        Token tokenType = TOK_UNKNOWN;
        switch (currentChar) {
            case '(': tokenType = TOK_LPAREN; break;
            case ')': tokenType = TOK_RPAREN; break;
            case '{': tokenType = TOK_LBRACE; break;
            case '}': tokenType = TOK_RBRACE; break;
            case '[': tokenType = TOK_LBRACKET; break;
            case ']': tokenType = TOK_RBRACKET; break;
            case ',': tokenType = TOK_COMMA; break;
            case '.': tokenType = TOK_DOT; break;
            case '!': tokenType = TOK_BANG; break;
        }

        tokens.push_back({tokenType, std::string(1, currentChar), m_line, currentCol});
        m_cursor++;
    }
    tokens.push_back({TOK_EOF, "", m_line, m_cursor - m_col_start + 1});
    return tokens;
}

} // namespace spaceship
