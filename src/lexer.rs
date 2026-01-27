use crate::token_data::{TokenType, Token};


#[derive(Debug)]
pub struct File {
    raw: String,
    tokens: Vec<Token>,
}

impl File {
    pub fn new(raw_data: String) -> Self {
        Self {
            raw: raw_data,
            tokens: vec![],
        }
    }
    

    pub fn tokenize(&mut self){
        let mut mode = 'n'; // n | s | i
        let mut str_start = 0;
        let mut int_start = 0;
        let mut position = 0;
        let mut bytes = self.raw.as_bytes();

        while position < bytes.len() {
            // string mode
            if mode == 's' {
                match bytes[position] {
                    b'"' => {
                        mode = 'n';
                        position += 1;
                        let value = &bytes[(str_start+1)..(position-1)];
                        self.tokens.push(Token {
                            token_type: TokenType::StringLiteral,
                            value: unsafe {std::str::from_utf8_unchecked(value)}.to_string(),
                            line: 0,
                            position: 1,
                        });
                    }

                    _ => {
                        position += 1;
                    }
                }
            }
            
            // handle integer mode.
            else if mode=='i' {
                match bytes[position] {
                    b'0'..=b'9' => {
                        position += 1;
                    }

                    b' ' | b'\n' => {
                        mode = 'n';
                        position += 1;
                        let value = &bytes[int_start..(position-1)];
                        self.tokens.push(Token {
                            token_type: TokenType::Integer,
                            value: unsafe {std::str::from_utf8_unchecked(value)}.to_string(),
                            line: 0,
                            position: 1,
                        });
 
                    }

                    _=> {
                        position += 1;
                    }
                }
            }

            // handle normal mode.
            else {
                match bytes[position] {
                    b'"' => {
                        mode = 's';
                        str_start = position;
                        position += 1;
                    }

                    b'0'..=b'9' => {
                        mode = 'i';
                        int_start = position;
                        position += 1;
                    }

                    _ => {
                        position += 1;
                    }
                }
            }
        }
        println!("{:#?}", self.tokens);
    }
}
