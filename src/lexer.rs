use std::process::id;

#[derive(Debug, Clone)]
pub enum TokenType {
    // Special
    Eof, // End of File

    // Identifiers & Literals
    Identifier(Vec<u8>),    // variable names, function names
    Integer,       // 123
    Float,         // 12.3
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
    Colon,       // :
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
    Float,
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
        let mut float_start = 0;
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

                        b':' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Colon,
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
                            // TODO: fix the getchar function for proper utf-8 decoding.
                            println!(
                                "Lexer: Unknown character '{}' at line {}",
                                getchar_from_bytes(&self.raw, self.cursor),
                                self.line
                            );
                            std::process::exit(1);
                        }
                    }
                }


                // String Mode: Tokenize strings.
                // TODO: add file ending edgecase.
                Mode::StringLiteral => {
                    if self.raw[self.cursor] == b'"' && self.raw[self.cursor-1] != b'\\' {
                        self.tokens.push(Token {
                            token_type: TokenType::StringLiteral,
                            start: str_start,
                            end: self.cursor - 1,
                            line: self.line,
                        });
                        self.cursor += 1;
                        mode = Mode::Normal;
                    } else {
                        self.cursor += 1;
                    }
                }
                
                // Integer Mode: tokenize Integers. and change mode to float if found dot.
                // TODO: add eof edgecase.
                Mode::Integer => {
                    match self.raw[self.cursor] {
                        // Enter float mode if encountered a dot in the integer.
                        b'.' => {
                            mode = Mode::Float;
                            float_start = int_start;
                            self.cursor += 1;
                        }
                        
                        // Tokenize the integer after it ends with a new line.
                        // IMPORTANT: Don't consume the newline just set the mode to Normal.
                        // Normal mode needs the newline yo keep track of line numbers.
                        b'\n' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Integer,
                                start: int_start,
                                end: self.cursor - 1,
                                line: self.line,
                            });
                            mode = Mode::Normal;
                        }

                        // Tokenize the Integer after it ends with a space.
                        b' ' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Integer,
                                start: int_start,
                                end: self.cursor - 1,
                                line: self.line,
                            });
                            mode = Mode::Normal;
                            self.cursor += 1;
                        }


                        // continue for valid numbers
                        b'0'..=b'9' => {
                            self.cursor += 1;
                        }
                        
                        // exit for everything else.
                        _ => {
                            println!(
                                "Lexer: invalid integer '{}' at line {}",
                                getchar_from_bytes(&self.raw, self.cursor),
                                self.line
                            );
                            std::process::exit(1);
                        }
                    }
                }

                // Float Mode: tokenize Float.
                // TODO: add eof edgecase.
                Mode::Float => {
                    match self.raw[self.cursor] {
                        // First dot was already consumed to enter float mode.
                        // the second one is invalid.
                        b'.' => {
                            println!(
                                "Lexer: too many floating points for integer at line {}",
                                self.line
                            );
                            std::process::exit(1);
                        }
                        
                        // Tokenize the Float after it ends with a new line.
                        // IMPORTANT: Don't consume the newline just set the mode to Normal.
                        // Normal mode needs the newline yo keep track of line numbers.
                        b'\n' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Float,
                                start: float_start,
                                end: self.cursor - 1,
                                line: self.line,
                            });
                            mode = Mode::Normal;
                        }

                        // Tokenize the Float after it ends with a space.
                        b' ' => {
                            self.tokens.push(Token {
                                token_type: TokenType::Float,
                                start: float_start,
                                end: self.cursor - 1,
                                line: self.line,
                            });
                            mode = Mode::Normal;
                            self.cursor += 1;
                        }


                        // continue for valid numbers
                        b'0'..=b'9' => {
                            self.cursor += 1;
                        }
                        
                        // exit for everything else.
                        _ => {
                            println!(
                                "Lexer: invalid Float '{}' at line {}",
                                getchar_from_bytes(&self.raw, self.cursor),
                                self.line
                            );
                            std::process::exit(1);
                        }
                    }
                }

                // Identifier Mode: Tokenize Identifiers and keywords.
                Mode::Identifier => {
                    match self.raw[self.cursor] {
                        // NOTE: first character of the Identifier/Keyword has already been 
                        // consumed by normal mode to enter Identifier mode. starting
                        // from second character, valid identifier/Keyword name should only have 
                        // a-z, A-Z, and 0-9 characters.
                        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' => {
                            self.cursor += 1;
                        }

                        // Tokenize Identifier/keyword after encountering 
                        // any other characters stated above.
                        // IMPORTANT: don't consume the character. leave it for normal mode.
                        _ => {
                            let (token_type, _) = get_token(&self.raw[id_start..self.cursor-1]);
                            match token_type {
                                // Keyword
                                Some(t) => {
                                    self.tokens.push(Token {
                                        token_type: t,
                                        start: id_start,
                                        end: self.cursor - 1,
                                        line: self.line,
                                    });
                                    mode = Mode::Normal;
                                }

                                // identifier
                                None => {
                                    self.tokens.push(Token {
                                        token_type: TokenType::Identifier(self.raw[id_start..self.cursor-1].to_vec()),
                                        start: id_start,
                                        end: self.cursor - 1,
                                        line: self.line,
                                    });
                                    mode = Mode::Normal;
                                }
                            }
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


        // handle not closed ("String) StringLiteral.
        // the mode 'Mode::StringLiteral' should still be active if it was not closed.
        if mode == Mode::StringLiteral {
            println!("Lexer: StringLiteral did not close.");
            std::process::exit(1);
        }

        println!("{:#?}", self.tokens);
    }
}
