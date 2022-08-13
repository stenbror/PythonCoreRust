
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };
use crate::parser::source_buffer::{ SourceBuffer };


// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////


pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>,
    trivia_collector: Box<Vec<Box<Trivia>>>,
    symbol: Option<Box<Token>>,
    parenthesis: Vec<char>,
    is_blank_line: bool,
    current_token_start: u32
}


// Implementing functions related to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    pub fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) ),
            trivia_collector: Box::new(Vec::new() ),
            symbol: Some( Box::new( Token::Empty ) ),
            parenthesis: Vec::new(),
            is_blank_line: true,
            current_token_start: 0
        }
    }

    fn is_ident_start_letter(&self, ch: char) -> bool {
        match &ch {
            '_' => true,
            _ => ch.is_alphabetic()
        }
    }

    fn is_ident_letter_or_digit(&self, ch: char) -> bool {
        match &ch {
            '_' => true,
            _ => ch.is_alphanumeric()
        }
    }

    fn handling_numbers(&mut self) -> Option<Token> {
        let mut buffer : String = String::new();
        let token_start_position = &self.get_position();
        let trivia = if self.trivia_collector.is_empty() { None } else
        {
            let mut trivia_tmp = self.trivia_collector.clone();
            trivia_tmp.reverse();
            Some( trivia_tmp )
        };
        let (a, b, c) = self.source_buffer.peek_three_chars();
        match (a, b) {
            ( '.' , '0' ..= '9') => {
                for i in 1..= 2 {
                    buffer.push( self.source_buffer.get_char().clone() );
                    self.source_buffer.advance();
                }
                while   match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                match &self.source_buffer.get_char() {
                    'e' | 'E' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '+' | '-' => {
                                buffer.push(self.source_buffer.get_char().clone());
                                self.source_buffer.advance();
                            },
                            _ => {}
                        }
                        while   match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {}
                }
                match &self.source_buffer.get_char() {
                    'j' | 'J' => {
                        buffer.push(self.source_buffer.get_char().clone());
                        self.source_buffer.advance();
                    },
                    _ => {}
                }
                self.trivia_collector = Box::new(Vec::new() );
                Some ( Token::AtomNumber(token_start_position.clone(), self.get_position(), trivia, Box::new( buffer )) )
            },
            ( '0', _  ) => {
                buffer.push( self.source_buffer.get_char().clone() );
                self.source_buffer.advance();
                match &self.source_buffer.get_char()  {
                    'x' | 'X' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' | '_' => {},
                            _ => {
                                panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                            }
                        }
                        while match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' => {},
                                    _ => {
                                        panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                    }
                                }
                                true
                            },
                            '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    'o' | 'O' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '0' ..= '7' | '_' => {},
                            _ => {
                                panic!("Syntax Error at {} - Expected digit or '_' after '0o' or '0O'!", &self.get_position())
                            }
                        }
                        while match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0' ..= '7' => {},
                                    _ => {
                                        panic!("Syntax Error at {} - Expected digit or '_' after '0o' or '0O'!", &self.get_position())
                                    }
                                }
                                true
                            },
                            '0' ..= '7' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    'b' | 'B' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '0' | '1' | '_' => {},
                            _ => {
                                panic!("Syntax Error at {} - Expected digit or '_' after '0b' or '0B'!", &self.get_position())
                            }
                        }
                        while match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0' | '1' => {},
                                    _ => {
                                        panic!("Syntax Error at {} - Expected digit or '_' after '0b' or '0B'!", &self.get_position())
                                    }
                                }
                                true
                            },
                            '0' | '1' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {
                        let mut non_zero = false;
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        while   match &self.source_buffer.get_char() {
                                    '0' => {
                                        buffer.push( self.source_buffer.get_char().clone() );
                                        self.source_buffer.advance();
                                        true
                                    },
                                    '1' ..= '9' => {
                                        non_zero = true;
                                        buffer.push( self.source_buffer.get_char().clone() );
                                        self.source_buffer.advance();
                                        true
                                    },
                                    _ => false
                                } {};
                        match &self.source_buffer.get_char() {
                            '.'  => {
                                non_zero = false;
                                buffer.push(self.source_buffer.get_char().clone());
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '_' => panic!("Syntax Error at {} - Expected digit after '.'!", &self.get_position()),
                                    _ => {}
                                }
                                while   match &self.source_buffer.get_char() {
                                    '_' => {
                                        buffer.push( self.source_buffer.get_char().clone() );
                                        self.source_buffer.advance();
                                        match &self.source_buffer.get_char() {
                                            '0'..='9' => true,
                                            _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                        }
                                    },
                                    '0' ..= '9' => {
                                        buffer.push( self.source_buffer.get_char().clone() );
                                        self.source_buffer.advance();
                                        true
                                    },
                                    _ => false
                                } {};
                            },
                            _ => {}
                        }
                        match &self.source_buffer.get_char() {
                            'e' | 'E' => {
                                non_zero = false;
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '+' | '-' => {
                                        buffer.push(self.source_buffer.get_char().clone());
                                        self.source_buffer.advance();
                                    },
                                    _ => {}
                                }
                                while   match &self.source_buffer.get_char() {
                                    '_' => {
                                        buffer.push( self.source_buffer.get_char().clone() );
                                        self.source_buffer.advance();
                                        match &self.source_buffer.get_char() {
                                            '0'..='9' => true,
                                            _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                        }
                                    },
                                    '0' ..= '9' => {
                                        buffer.push( self.source_buffer.get_char().clone() );
                                        self.source_buffer.advance();
                                        true
                                    },
                                    _ => false
                                } {};
                            },
                            _ => {}
                        }
                        match &self.source_buffer.get_char() {
                            'j' | 'J' => {
                                non_zero = false;
                                buffer.push(self.source_buffer.get_char().clone());
                                self.source_buffer.advance();
                            },
                            _ => {}
                        }
                        match &non_zero {
                            true => panic!("Syntax Error at {} - Leading zero in a integer number is not allowed'!", &self.get_position()),
                            _ => {}
                        }
                    }
                }
                self.trivia_collector = Box::new(Vec::new() );
                Some ( Token::AtomNumber(token_start_position.clone(), self.get_position(), trivia, Box::new( buffer )) )
            },
            ( '1' ..= '9', _ ) => {
                while   match &self.source_buffer.get_char() {
                    '_' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '0'..='9' => true,
                            _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                        }
                    },
                    '0' ..= '9' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        true
                    },
                    _ => false
                } {};
                match &self.source_buffer.get_char() {
                    '.'  => {
                        buffer.push(self.source_buffer.get_char().clone());
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '_' => panic!("Syntax Error at {} - Expected digit after '.'!", &self.get_position()),
                            _ => {}
                        }
                        while   match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {}
                }
                match &self.source_buffer.get_char() {
                    'e' | 'E' => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        match &self.source_buffer.get_char() {
                            '+' | '-' => {
                                buffer.push(self.source_buffer.get_char().clone());
                                self.source_buffer.advance();
                            },
                            _ => {}
                        }
                        while   match &self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => panic!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position())
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {}
                }
                match &self.source_buffer.get_char() {
                    'j' | 'J' => {
                        buffer.push(self.source_buffer.get_char().clone());
                        self.source_buffer.advance();
                    },
                    _ => {}
                }
                self.trivia_collector = Box::new(Vec::new() );
                Some ( Token::AtomNumber(token_start_position.clone(), self.get_position(), trivia, Box::new( buffer )) )
            }, _ => None
        }
    }

    fn handling_strings(&mut self, prefix: Option<String>, start_pos: &u32) -> Option<Token> {
        let mut buffer : String = String::new();
        let trivia = if self.trivia_collector.is_empty() { None } else
        {
            let mut trivia_tmp = self.trivia_collector.clone();
            trivia_tmp.reverse();
            Some( trivia_tmp )
        };
        match *self.source_buffer.get_char() {
            '\'' |
            '"' => {
                match &self.source_buffer.peek_three_chars() {
                    ( '"', '"', '"' ) => {
                        for i in 1 ..= 3 {
                            buffer.push('"');
                            self.source_buffer.advance();
                        }
                        while match &self.source_buffer.peek_three_chars() {
                            ( '"', '"', '"' ) => {
                                for i in 1 ..= 3 {
                                    buffer.push('"');
                                    self.source_buffer.advance();
                                }
                                false
                            }, // TODO: Handle end of file inside tripple string in interactive mode!
                            _ => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            }
                        } {};
                    },
                    ( '"', '"' , _ ) => { // Empty ""
                        for i in 1 ..= 2 {
                            buffer.push('"');
                            self.source_buffer.advance();
                        }
                    },
                    ( '"', _ , _ ) => { // Single quote string with "
                        buffer.push('"');
                        self.source_buffer.advance();
                        while match &self.source_buffer.get_char() {
                            '"' => {
                                buffer.push('"');
                                self.source_buffer.advance();
                                false
                            },
                            '\0' |
                            '\r' |
                            '\n' => {
                                panic!("Syntax Error at {} - Unterminated single quote string literal!", &self.get_position())
                            },
                            _ => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            }
                        } {};
                    },
                    ( '\'', '\'', '\'' ) => {
                        for i in 1 ..= 3 {
                            buffer.push('\'');
                            self.source_buffer.advance();
                        }
                        while match &self.source_buffer.peek_three_chars() {
                            ( '\'', '\'', '\'' ) => {
                                for i in 1 ..= 3 {
                                    buffer.push('\'');
                                    self.source_buffer.advance();
                                }
                                false
                            }, // TODO: Handle end of file inside tripple string in interactive mode!
                            _ => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            }
                        } {};
                    },
                    ( '\'', '\'', _ ) => { // Empty ''
                        for i in 1 ..= 2 {
                            buffer.push('\'');
                            self.source_buffer.advance();
                        }
                    },
                    ( '\'', _ , _ ) => { // Single quote string with '
                        buffer.push('\'');
                        self.source_buffer.advance();
                        while match &self.source_buffer.get_char() {
                            '\'' => {
                                buffer.push('\'');
                                self.source_buffer.advance();
                                false
                            },
                            '\0' |
                            '\r' |
                            '\n' => {
                                panic!("Syntax Error at {} - Unterminated single quote string literal!", &self.get_position())
                            },
                            _ => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                self.source_buffer.advance();
                                true
                            }
                        } {};
                    },
                    _ => { }
                }
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::AtomString(*start_pos, self.get_position(), trivia, Box::new( buffer ), prefix) )
            },
            _ => None
        }
    }

    fn keywords_or_name_literal(&mut self) -> Option<Token> {
        let token_start_position = &self.get_position();
        let mut buffer : String = String::new();
        match &self.is_ident_start_letter(*self.source_buffer.get_char()) {
            true => {
                while self.is_ident_letter_or_digit(*self.source_buffer.get_char()) {
                    buffer.push(*self.source_buffer.get_char());
                    &self.source_buffer.advance();
                }
                match self.is_reserved_keyword(&token_start_position, &self.get_position(), &buffer.as_str()) {
                    Some( s ) => {
                        self.trivia_collector = Box::new(Vec::new() );
                        Some(s)
                    },
                    None => {
                        match *self.source_buffer.get_char() {
                            '\'' |
                            '"' => {
                                match buffer.as_str() {
                                    "r" |
                                    "u" |
                                    "R" |
                                    "U" |
                                    "f" |
                                    "F" |
                                    "fr" |
                                    "Fr" |
                                    "fR" |
                                    "FR" |
                                    "rf" |
                                    "rF" |
                                    "Rf" |
                                    "RF" => {
                                        self.handling_strings(Some( buffer ), token_start_position)
                                    },
                                    _ => {
                                        let trivia = if self.trivia_collector.is_empty() { None } else {
                                            let mut trivia_tmp = self.trivia_collector.clone();
                                            trivia_tmp.reverse();
                                            Some( trivia_tmp )
                                        };
                                        self.trivia_collector = Box::new(Vec::new() );
                                        Some( Token::AtomName(*token_start_position, self.get_position(), trivia, Box::new( buffer ) ))
                                    }
                                }
                            },
                            _ => {
                                let trivia = if self.trivia_collector.is_empty() { None } else {
                                    let mut trivia_tmp = self.trivia_collector.clone();
                                    trivia_tmp.reverse();
                                    Some( trivia_tmp )
                                };
                                self.trivia_collector = Box::new(Vec::new() );
                                Some( Token::AtomName(*token_start_position, self.get_position(), trivia, Box::new( buffer ) ))
                            }
                        }
                    }
                }
            },
            _ => None
        }
    }

    pub fn advance(&mut self) -> () {
        self.current_token_start = *self.source_buffer.get_position();
        match self.outer_loop() {
            Some( x ) => {
                self.symbol = Some( Box::new( x ) )
            },
            _ => {}
        }
    }

    pub fn get_symbol(&self) -> Box<Token> {
        match &self.symbol {
            Some(s) => {
                Box::new(*s.clone())
            },
            None => {
                Box::new ( Token::Empty )
            }
        }
    }

    pub fn get_position(&self) -> u32 {
        self.current_token_start.clone()
    }

    /// This method checks for valid operator or delimiter including pairing parenthezis if present before returning token or Option<Token> = None.
    fn is_operator_or_delimiter(&mut self, start_pos: &u32, a: &char, b: &char, c: &char) -> Option<Token> {
        match ( &a, &b, &c ) {
            ( '*', '*', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPowerAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '*', '*', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPower(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '*', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMulAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '*', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMul(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', '/', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyFloorDivAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', '/', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyFloorDiv(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyDivAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyDiv(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '.', '.', '.' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyElipsis(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '.', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyDot(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '<', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftLeftAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '<', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftLeft(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyLessEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '>', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyNotEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyLess(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', '>', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftRightAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', '>', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftRight(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyGreaterEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyGreater(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '+', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPlusAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '+', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPlus(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '-', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMinusAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '-', '>', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyArrow(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '-', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMinus(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '%', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyModuloAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '%', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyModulo(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '@', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMatriceAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '@', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMatrice(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '=', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '=', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '!', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyNotEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '&', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitAndAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '&', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitAnd(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '|', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitOrAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '|', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitOr(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '^', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitXorAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '^', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitXor(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '(', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                &self.parenthesis.push(')');
                Some( Token::PyLeftParen(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '[', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                &self.parenthesis.push(']');
                Some( Token::PyLeftBracket(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '{', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                &self.parenthesis.push('}');
                Some( Token::PyLeftCurly(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ')', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                match &self.parenthesis.last() {
                    Some( ')' ) => { self.parenthesis.pop(); },
                    _ => {
                        panic!("Syntax Error at {} - Mismatch in parenthesis, expected ')'!", &self.get_position())
                    }
                }
                Some( Token::PyRightParen(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ']', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                match &self.parenthesis.last() {
                    Some( ']' ) => { self.parenthesis.pop(); },
                    _ => {
                        panic!("Syntax Error at {} - Mismatch in parenthesis, expected ']'!", &self.get_position())
                    }
                }
                Some( Token::PyRightBracket(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '}', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                match &self.parenthesis.last() {
                    Some( '}' ) => { self.parenthesis.pop(); },
                    _ => {
                        panic!("Syntax Error at {} - Mismatch in parenthesis, expected '{}'!", &self.get_position(), '}')
                    }
                }
                Some( Token::PyRightCurly(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ':', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyColonAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ':', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyColon(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ';', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PySemiColon(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ',', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyComa(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '~', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    Some( trivia_tmp )
                };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitInvert(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( _ , _ , _ ) => {
                None
            }
        }
    }

    /// This method checks for reserved keywords or atom name literal and provides token with position and trivia collected in fron ot token
    fn is_reserved_keyword(&mut self, start_pos: &u32, end_pos: &u32, buffer: &str) -> Option<Token> {
        let trivia = if self.trivia_collector.is_empty() { None } else
        {
            let mut trivia_tmp = self.trivia_collector.clone();
            trivia_tmp.reverse();
            self.trivia_collector = Box::new( Vec::new() );
            Some( trivia_tmp )
        };
        match &*buffer {
            "False" => Some( Token::PyFalse(*start_pos, *end_pos, trivia) ),
            "None" => Some( Token::PyNone(*start_pos, *end_pos, trivia) ),
            "True" => Some( Token::PyTrue(*start_pos, *end_pos, trivia) ),
            "and" => Some ( Token::PyAnd(*start_pos, *end_pos, trivia) ),
            "as" => Some ( Token::PyAs(*start_pos, *end_pos, trivia) ),
            "assert" => Some ( Token::PyAssert(*start_pos, *end_pos, trivia) ),
            "async" => Some ( Token::PyAsync(*start_pos, *end_pos, trivia) ),
            "await" => Some ( Token::PyAwait(*start_pos, *end_pos, trivia) ),
            "break" => Some ( Token::PyBreak(*start_pos, *end_pos, trivia) ),
            "class" => Some ( Token::PyClass(*start_pos, *end_pos, trivia) ),
            "continue" => Some ( Token::PyContinue(*start_pos, *end_pos, trivia) ),
            "def" => Some ( Token::PyDef(*start_pos, *end_pos, trivia) ),
            "del" => Some ( Token::PyDel(*start_pos, *end_pos, trivia) ),
            "elif" => Some ( Token::PyElif(*start_pos, *end_pos, trivia) ),
            "else" => Some ( Token::PyElse(*start_pos, *end_pos, trivia) ),
            "except" => Some ( Token::PyExcept(*start_pos, *end_pos, trivia) ),
            "finally" => Some ( Token::PyFinally(*start_pos, *end_pos, trivia) ),
            "for" => Some ( Token::PyFor(*start_pos, *end_pos, trivia) ),
            "from" => Some ( Token::PyFrom(*start_pos, *end_pos, trivia) ),
            "global" => Some ( Token::PyGlobal(*start_pos, *end_pos, trivia) ),
            "if" => Some ( Token::PyIf(*start_pos, *end_pos, trivia) ),
            "import" => Some ( Token::PyImport(*start_pos, *end_pos, trivia) ),
            "in" => Some ( Token::PyIn(*start_pos, *end_pos, trivia) ),
            "is" => Some ( Token::PyIs(*start_pos, *end_pos, trivia) ),
            "lambda" => Some ( Token::PyLambda(*start_pos, *end_pos, trivia) ),
            "nonlocal" => Some ( Token::PyNonLocal(*start_pos, *end_pos, trivia) ),
            "not" => Some ( Token::PyNot(*start_pos, *end_pos, trivia) ),
            "or" => Some ( Token::PyOr(*start_pos, *end_pos, trivia) ),
            "pass" => Some ( Token::PyPass(*start_pos, *end_pos, trivia) ),
            "raise" => Some ( Token::PyRaise(*start_pos, *end_pos, trivia) ),
            "return" => Some ( Token::PyReturn(*start_pos, *end_pos, trivia) ),
            "try" => Some ( Token::PyTry(*start_pos, *end_pos, trivia) ),
            "while" => Some ( Token::PyWhile(*start_pos, *end_pos, trivia) ),
            "with" => Some ( Token::PyWith(*start_pos, *end_pos, trivia) ),
            "yield" => Some ( Token::PyYield(*start_pos, *end_pos, trivia) ),
            _ => None
        }
    }

    fn handle_type_comment(&mut self) -> Option<Token> {
        let token_start_position = &self.get_position();
        match *self.source_buffer.get_char() {
            '#' => {
                let mut buffer : String = String::new();
                let token_start_position = &self.get_position();
                buffer.push( self.source_buffer.get_char().clone() );
                self.source_buffer.advance();
                while match *self.source_buffer.get_char() {
                    '\r' | '\n' | '\0' => false,
                    _ => {
                        buffer.push( self.source_buffer.get_char().clone() );
                        self.source_buffer.advance();
                        true
                    }
                } {};
                match buffer.as_str().starts_with("# type:") {
                    true => {
                        let trivia = if self.trivia_collector.is_empty() { None } else
                        {
                            let mut trivia_tmp = self.trivia_collector.clone();
                            trivia_tmp.reverse();
                            self.trivia_collector = Box::new( Vec::new() );
                            Some( trivia_tmp )
                        };
                        Some( Token::TypeComment(*token_start_position, self.get_position(), trivia, Box::new( buffer ) ) )
                    },
                    _ => {
                        self.trivia_collector.push(Box::new( Trivia::Comment(*token_start_position, self.get_position(), buffer) ) );
                        None
                    }
                }
            },
            _ => None
        }
    }

    fn handle_newlines(&mut self) -> Option<Token> {
        let token_start_position = &self.get_position();
        match &self.source_buffer.peek_three_chars() {
            ( '\r', '\n', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance(); }
                match ( &self.is_blank_line, &self.parenthesis.is_empty() ) {
                    ( true, false ) => {
                        let trivia_entry = Box::new( Trivia::Newline(*token_start_position, self.get_position(), '\r', '\n') );
                        self.trivia_collector.push(trivia_entry);
                        None
                    },
                    _ => {
                        let trivia = if self.trivia_collector.is_empty() { None } else
                        {
                            let mut trivia_tmp = self.trivia_collector.clone();
                            trivia_tmp.reverse();
                            self.trivia_collector = Box::new( Vec::new() );
                            Some( trivia_tmp )
                        };
                        Some( Token::Newline(*token_start_position, self.get_position(), trivia, '\r', '\n' ) )
                    }
                }
            },
            ( '\r', _ , _  ) |
            ( '\n', _ , _  ) => {
                let ch = self.source_buffer.get_char().clone();
                self.source_buffer.advance();
                match ( &self.is_blank_line, &self.parenthesis.is_empty() ) {
                    ( true, false ) => {
                        let trivia_entry = Box::new( Trivia::Newline(*token_start_position, self.get_position(), ch, ' ') );
                        self.trivia_collector.push(trivia_entry);
                        None
                    },
                    _ => {
                        let trivia = if self.trivia_collector.is_empty() { None } else
                        {
                            let mut trivia_tmp = self.trivia_collector.clone();
                            trivia_tmp.reverse();
                            self.trivia_collector = Box::new( Vec::new() );
                            Some( trivia_tmp )
                        };
                        Some( Token::Newline(*token_start_position, self.get_position(), trivia, ch, ' ' ) )
                    }
                }
            },
            _ => None
        }
    }

    fn handle_end_of_file(&mut self) -> Option<Token> {
        let token_start_position = &self.get_position();
        match *self.source_buffer.get_char() {
            '\0' => {
                let trivia = if self.trivia_collector.is_empty() { None } else
                {
                    let mut trivia_tmp = self.trivia_collector.clone();
                    trivia_tmp.reverse();
                    self.trivia_collector = Box::new( Vec::new() );
                    Some( trivia_tmp )
                };
                Some( Token::EOF( *token_start_position, trivia) )
            },
            _ => None
        }
    }

    fn handle_whitespace(&mut self) -> Option<Token> {
        match *self.source_buffer.get_char() {
            ' ' | '\t' => {
                    while
                        match *self.source_buffer.get_char() {
                            ' ' => {
                                let token_start_position = &self.get_position();
                                &self.source_buffer.advance();
                                while match *self.source_buffer.get_char() { ' ' =>  { &self.source_buffer.advance(); true }, _ => false} {};
                                let trivia = Box::new( Trivia::Whitespace(*token_start_position, self.get_position(), ' ') );
                                self.trivia_collector.push(trivia);
                                true
                            },
                            '\t' => {
                                let token_start_position = &self.get_position();
                                &self.source_buffer.advance();
                                while match *self.source_buffer.get_char() { ' ' =>  { &self.source_buffer.advance(); true }, _ => false} {};
                                let trivia = Box::new( Trivia::Whitespace(*token_start_position, self.get_position(), '\t') );
                                self.trivia_collector.push(trivia);
                                true
                            },
                            _ => false
                        } {};
                self.inner_loop()
            },
            _ => None
        }
    }

    fn handle_line_continuation(&mut self) -> () {
        let token_start_position = &self.get_position();
        match &self.source_buffer.peek_three_chars() {
            ( '\\', '\r', '\n' ) => {
                let mut a = &self.source_buffer.get_char().clone();
                &self.source_buffer.advance();
                let mut b = &self.source_buffer.get_char().clone();
                &self.source_buffer.advance();
                let mut c = &self.source_buffer.get_char().clone();
                &self.source_buffer.advance();
                let trivia = Box::new( Trivia::LineContinuation(*token_start_position, self.get_position(), *a, *b, *c) );
                self.trivia_collector.push(trivia );
            },
            ( '\\', '\r', _  )  |
            ( '\\', '\n', _ ) => {
                let mut a = &self.source_buffer.get_char().clone();
                &self.source_buffer.advance();
                let mut b = &self.source_buffer.get_char().clone();
                &self.source_buffer.advance();
                let trivia = Box::new( Trivia::LineContinuation(*token_start_position, self.get_position(), *a, *b, ' ') );
                self.trivia_collector.push(trivia );
            },
            _ => { }
        }
    }

    fn inner_loop(&mut self) -> Option<Token> {
        match self.handle_whitespace() { // Handle whitespace
            Some( x ) => Some( x ),
            _ => {
                match self.handle_type_comment() { // Handle type comment or plain comment
                    Some( x ) => Some( x ),
                    _ => {
                        match self.handle_end_of_file() { // Handle end of file
                            Some( x ) => Some( x ),
                            _ => {
                                match  self.is_ident_start_letter( self.source_buffer.get_char().clone() )       { // Reserved keyword or name literal
                                    true => {
                                        let token_start_position = &self.get_position();
                                        let mut buffer : String = String::new();
                                        while   match self.is_ident_letter_or_digit( self.source_buffer.get_char().clone() ) {
                                                    true => {
                                                        buffer.push( self.source_buffer.get_char().clone() );
                                                        &self.source_buffer.advance();
                                                        true
                                                    },
                                            _ => false
                                        } {};

                                        self.is_reserved_keyword(token_start_position, &self.source_buffer.get_position().clone(), buffer.as_str())
                                    },
                                    _ => {
                                        match self.handle_newlines() { // Handle newlines, either as a token or trivia
                                            Some( x ) => Some( x ),
                                            _ => {
                                                match self.handling_numbers() { // Handle numbers
                                                    Some( x ) => Some( x ),
                                                    _ => {
                                                        match self.handling_strings(None, &self.source_buffer.get_position().clone()) { // Handle strings without prefix.
                                                            Some( x ) => Some( x.clone() ),
                                                            _ => {
                                                                self.handle_line_continuation(); // Handle line continuation

                                                                let token_start_position = &self.get_position();
                                                                let (a, b, c) = self.source_buffer.peek_three_chars();
                                                                let a1 = a.clone();
                                                                let b1 = c.clone();
                                                                let c1 = a.clone();

                                                                match self.is_operator_or_delimiter(token_start_position, &a1, &b1, &c1) { // Operator or delimiter
                                                                    Some( x ) => Some( x.clone() ),
                                                                    _ => {
                                                                        panic!("Syntax Error at {} - Found illegal character!", &self.get_position())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                     }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn outer_loop(&mut self) -> Option<Token> {
        self.inner_loop()
    }

}