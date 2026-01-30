#[derive(Debug, Clone)]
pub enum TokenType {
    // Special
    Eof, // End of File

    // Identifiers & Literals
    Identifier,    // variable names, function names
    Integer,       // 123
    StringLiteral, // "hello"

    // Assignment Operators
    Assign,      // =
    PlusAssign,  // +=
    MinusAssign, // -=

    // Comparison Operators
    Equal,        // ==
    NotEqual,     // !=
    Less,         // <
    LessEqual,    // <=
    Greater,      // >
    GreaterEqual, // >=

    // Mathematical Operators
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /

    // Logical Operators
    And,  // &
    Or,   // |
    Bang, // !

    // Delimiters (The "Glue" of syntax)
    Comma,       // ,
    Semicolon,   // ;
    LeftParen,   // (
    RightParen,  // )
    LeftBrace,   // {
    RightBrace,  // }
    LeftSquare,  // [
    RightSquare, // ]

    // Keywords
    True,
    False,
    Function,
    If,
    Else,
    For,
    While,
    Return,
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Normal,
    StringLiteral,
    Integer,
    Comment,
    Identifier,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

pub fn get_token(bytes: &[u8]) -> (Option<TokenType>, String) {
    let value = unsafe { std::str::from_utf8_unchecked(bytes) }.to_string();
    let token_type = match bytes {
        // keywords.
        b"function" => Some(TokenType::Function),
        b"if" => Some(TokenType::If),
        b"else" => Some(TokenType::Else),
        b"for" => Some(TokenType::For),
        b"while" => Some(TokenType::While),
        b"return" => Some(TokenType::Return),
        b"true" => Some(TokenType::True),
        b"false" => Some(TokenType::False),
        _ => None,
    };
    (token_type, value)
}

fn getchar_from_bytes(bytes: &[u8], position: usize) -> char {
    // utf-8 can have a maximum of 4 bytes for multi byte character like 'π'
    let end_position = (position + 4).min(bytes.len());
    let data = String::from_utf8_lossy(&bytes[position..end_position]);
    data.chars().next().unwrap()
}


#[derive(Debug)]
pub struct File {
    raw: Vec<u8>,
    tokens: Vec<Token>,
    cursor: usize,
    line: usize,
}

impl File {
    pub fn new(raw_data: Vec<u8>) -> Self {
        Self {
            raw: raw_data,
            tokens: vec![],
            cursor: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) {
        let mut mode = Mode::Normal;
        let mut str_start = 0;
        let mut int_start = 0;
        let mut id_start  = 0;

        while self.cursor < self.raw.len() {
            match mode {
                // Normal mode checks each char and tokenize them Or changes the
                // Mode to something else accordingly yo handle conplex tokens.
                Mode::Normal => {
                    match self.raw[self.cursor] {
                        // increase the line number in self.line.
                        // IMPORTANT: make sure all the other modes doesn't consume the
                        // newline character to preserve your sanity later on while debugging.
                        b'\n' => {
                            self.line += 1;
                            self.cursor += 1;
                        }

                        // ignore space and tabs
                        b' ' | b'\t' => {
                            self.cursor += 1;
                        }

                        // tokenize simple 1-2 character tokens.
                        // =, +=, -=
                        b'=' => {
                            if self.raw[self.cursor + 1] == b'=' {
                                self.tokens.push(Token {
                                    token_type: TokenType::Equal,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 2;
                            } else {
                                self.tokens.push(Token {
                                    token_type: TokenType::Assign,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 1;
                            }
                        }

                        b'+' => {
                            if self.raw[self.cursor + 1] == b'=' {
                                self.tokens.push(Token {
                                    token_type: TokenType::PlusAssign,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 2;
                            } else {
                                self.tokens.push(Token {
                                    token_type: TokenType::Plus,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 1;
                            }
                        }

                        b'-' => {
                            if self.raw[self.cursor + 1] == b'=' {
                                self.tokens.push(Token {
                                    token_type: TokenType::MinusAssign,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 2;
                            } else {
                                self.tokens.push(Token {
                                    token_type: TokenType::Minus,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 1;
                            }
                        }

                        b'<' => {
                            if self.raw[self.cursor + 1] == b'=' {
                                self.tokens.push(Token {
                                    token_type: TokenType::LessEqual,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 2;
                            } else {
                                self.tokens.push(Token {
                                    token_type: TokenType::Less,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 1;
                            }
                        }

                        b'>' => {
                            if self.raw[self.cursor + 1] == b'=' {
                                self.tokens.push(Token {
                                    token_type: TokenType::GreaterEqual,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 2;
                            } else {
                                self.tokens.push(Token {
                                    token_type: TokenType::Greater,
                                    start: self.cursor,
                                    end: self.cursor + 1,
                                    line: self.line,
                                });
                                self.cursor += 1;
                            }
                        }

                        b'*' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Asterisk,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b'/' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Slash,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b',' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Comma,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b';' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Semicolon,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b'(' => {
                            self.tokens.push(Token {
                                token_type: TokenType::LeftParen,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b')' => {
                            self.tokens.push(Token {
                                token_type: TokenType::RightParen,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b'{' => {
                            self.tokens.push(Token {
                                token_type: TokenType::LeftBrace,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b'}' => {
                            self.tokens.push(Token {
                                token_type: TokenType::RightBrace,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b'[' => {
                            self.tokens.push(Token {
                                token_type: TokenType::LeftSquare,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        b']' => {
                            self.tokens.push(Token {
                                token_type: TokenType::RightSquare,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

                        
                        b'&' => {
                            self.tokens.push(Token {
                                token_type: TokenType::And,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

 
                        b'|' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Or,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }

 
                        b'!' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Bang,
                                start: self.cursor,
                                end: self.cursor + 1,
                                line: self.line,
                            });
                            self.cursor += 1;
                        }


                        // handle conplex tokens like String, Integer etc. by changing
                        // the mode accordingly.
                        
                        // Enter String mode.
                        b'"' => {
                            mode = Mode::StringLiteral;
                            self.cursor += 1;
                            // Note: add starting position after the increase so the 
                            // quote gets skipped.
                            str_start = self.cursor;
                        }
                        
                        // Enter Integer mode.
                        b'0'..=b'9' => {
                            mode = Mode::Integer;
                            int_start = self.cursor;
                            self.cursor += 1;
                        }
                        
                        // Enter Identifier Mode.
                        // Note: Identifier Mode also handles keywords.
                        b'a'..=b'z' | b'A'..=b'Z' =>{
                            mode = Mode::Identifier;
                            id_start = self.cursor;
                            self.cursor += 1;
                        }

                        _ => {
                            println!(
                                "Lexer: Unknown character '{}' at line {}",
                                getchar_from_bytes(&self.raw, self.cursor),
                                self.line
                            );
                            std::process::exit(1);
                        }
                    }
                }

                _ => {
                    println!("Lexer: State machine 'mode' got corrupted at runtime.");
                    println!("Unknown mode: {:#?}", mode);
                    std::process::exit(1);
                }
            }
        }

        println!("{:#?}", self.tokens);
    }
}
