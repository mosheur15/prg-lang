#[derive(Debug)]
pub enum TokenType {
    // Special
    Illegal,      // For characters the lexer doesn't recognize
    Eof,          // End of File

    // Identifiers & Literals
    Identifier,    // variable names, function names
    Integer,       // 123
    StringLiteral, // "hello"

    // Assignment Operators
    Assign,        // =
    PlusAssign,    // +=
    MinusAssign,   // -=

    // Comparison Operators
    Equal,         // ==
    NotEqual,      // !=
    Less,          // <
    LessEqual,     // <=
    Greater,       // >
    GreaterEqual,  // >=

    // Mathematical Operators
    Plus,          // +
    Minus,         // -
    Asterisk,      // *
    Slash,         // /

    // Logical Operators
    And,           // &&
    Or,            // ||
    Bang,          // !

    // Delimiters (The "Glue" of syntax)
    Comma,         // ,
    Semicolon,     // ;
    LeftParen,     // (
    RightParen,    // )
    LeftBrace,     // {
    RightBrace,    // }

    // Keywords
    Function,
    If,
    Else,
    For,
    While,
    Return,

    // special placeholder.
    NotFound,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    StringLiteral,
    Integer,
    Comment,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: i32,
    pub position: i32,
}

