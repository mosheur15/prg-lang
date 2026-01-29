use crate::token_data::{TokenType, Token, Mode};


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
        let data = self.raw.as_bytes();
        let mut mode = Mode::Normal;
        let mut count = 0;

        let mut str_start = 0;
        let mut int_start = 0;

        while count < data.len() {
            match mode {
                // handle logic for String mode.
                Mode::StringLiteral => {
                    match data[count] {
                        b'"' => {
                            if data[count-1] != b'\\' {
                                mode = Mode::Normal;
                                let value = &data[str_start..count];
                                count += 1;
                                self.tokens.push(Token {
                                    token_type: TokenType::StringLiteral,
                                    value: unsafe {std::str::from_utf8_unchecked(&value)}.to_string(),
                                    line: 0,
                                    position: str_start as i32,
                                });
                            }
                            else {
                                count += 1;
                            }
                        }
                        _ => {
                            count += 1;
                        }
                    }
                }
                
                // handle Integer mode.
                Mode::Integer => {
                    match data[count] {
                        // check if the byte is a digit 0-9.
                        b'0'..=b'9' => {
                            count += 1;
                        }
                        
                        // change mode to normal and collect the Integer if the 
                        // byte is space or newline.
                        b' ' | b'\n' => {
                            mode = Mode::Normal;
                            let value = &data[int_start..count];
                            count += 1;
                            self.tokens.push(Token {
                                token_type: TokenType::Integer,
                                value: unsafe {std::str::from_utf8_unchecked(value)}.to_string(),
                                line: 0,
                                position: int_start as i32,
                            });
                        }
                        _ => {
                            println!("illegal integer {} at position {}",
                                unsafe{std::str::from_utf8_unchecked(&data[count..count+1])}.to_string(),
                                count
                            );
                            std::process::exit(2);
                        }
                    }
                }

                // Normal mode.
                _ => {
                    match data[count] {
                        // if theres double quotes enter string mode.
                        b'"' => {
                            mode = Mode::StringLiteral;
                            count += 1;
                            str_start = count;
                        }

                        // if theres number 0-9 enter Integer mode.
                        b'0'..=b'9' => {
                            mode = Mode::Integer;
                            int_start = count;
                            count += 1;
                        }

                        _ => {
                            count += 1;
                        }
                    }
                }
            }
        };
        
        // Handle not closed modes.
        if mode == Mode::Integer {
            self.tokens.push(Token {
                token_type: TokenType::Integer,
                value: unsafe{std::str::from_utf8_unchecked(&data[int_start..count])}.to_string(),
                line: 0,
                position: count as i32,
            });
        }
        println!("{:#?}", self.tokens);
    }
}
