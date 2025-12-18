#include "lexer.h"
#include <iostream>
#include <cctype>

namespace orbit {

Lexer::Lexer(const std::string& source) : m_source(source), m_cursor(0) {}

std::vector<std::pair<Token, std::string>> Lexer::tokenize() {
    std::vector<std::pair<Token, std::string>> tokens;
    while (m_cursor < m_source.length()) {
        char currentChar = m_source[m_cursor];

        if (isspace(currentChar)) {
            m_cursor++;
            continue;
        }

        if (isalpha(currentChar)) {
            std::string identifier = "";
            while (m_cursor < m_source.length() && (isalnum(m_source[m_cursor]) || m_source[m_cursor] == '_')) {
                identifier += m_source[m_cursor];
                m_cursor++;
            }

            if (identifier == "if") tokens.push_back({TOK_IF, "if"});
            else if (identifier == "for") tokens.push_back({TOK_FOR, "for"});
            else if (identifier == "check") tokens.push_back({TOK_CHECK, "check"});
            else if (identifier == "except") tokens.push_back({TOK_EXCEPT, "except"});
            else tokens.push_back({TOK_IDENTIFIER, identifier});
            continue;
        }

        if (isdigit(currentChar)) {
            std::string number = "";
            while (m_cursor < m_source.length() && isdigit(m_source[m_cursor])) {
                number += m_source[m_cursor];
                m_cursor++;
            }
            tokens.push_back({TOK_LITERAL_INTEGER, number});
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
            tokens.push_back({TOK_LITERAL_STRING, str});
            continue;
        }

        if (currentChar == '.') {
            if (m_cursor + 4 < m_source.length() && m_source.substr(m_cursor, 5) == ".then") {
                tokens.push_back({TOK_THEN, ".then"});
                m_cursor += 5;
            } else {
                tokens.push_back({TOK_DOT, "."});
                m_cursor++;
            }
            continue;
        }

        switch (currentChar) {
            case '(': tokens.push_back({TOK_LPAREN, "("}); break;
            case ')': tokens.push_back({TOK_RPAREN, ")"}); break;
            case '{': tokens.push_back({TOK_LBRACE, "{"}); break;
            case '}': tokens.push_back({TOK_RBRACE, "}"}); break;
            case ',': tokens.push_back({TOK_COMMA, ","}); break;
            case ':': tokens.push_back({TOK_COLON, ":"}); break;
            case '[': tokens.push_back({TOK_LBRACKET, "["}); break;
            case ']': tokens.push_back({TOK_RBRACKET, "]"}); break;
            default:
                // Handle unknown characters or errors
                m_cursor++;
                continue;
        }
        m_cursor++;
    }
    tokens.push_back({TOK_EOF, ""});
    return tokens;
}

}
