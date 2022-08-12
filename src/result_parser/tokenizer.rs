use crate::parser::trivias::Trivia;
use crate::result_parser::source_buffer::SourceBuffer;
use crate::result_parser::source_buffer::SourceBufferFunctionality;
use crate::Token;

pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>,
    token_start_position: u32,
    parenthesis: Vec<char>
}


trait Tokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer;
    fn get_symbol(&mut self) -> Result<Box<Token>, String>;
    fn get_position(&self) -> u32;
}


impl Tokenizer for PythonCoreTokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) ),
            token_start_position: 0u32,
            parenthesis: Vec::new(),
        }
    }

    fn get_symbol(&mut self) -> Result<Box<Token>, String> {

        let mut trivia_collector : Box<Vec<Box<Trivia>>> = Box::new( Vec::new() );

        self.token_start_position = self.source_buffer.get_position(); // Saves starts of current token symbol.


        match self.source_buffer.peek_three_chars() {
            ( '*', '*', '=' ) => {
                for i in 1 ..= 3 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyPowerAssign(self.token_start_position, self.source_buffer.get_position(),
                                                  match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '*', '*', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyPower(self.token_start_position, self.source_buffer.get_position(),
                                                  match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '*', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyMulAssign(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '*', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyMul(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '/', '/', '=' ) => {
                for i in 1 ..= 3 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyFloorDivAssign(self.token_start_position, self.source_buffer.get_position(),
                                                  match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '/', '/', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyFloorDiv(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '/', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyDivAssign(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '/', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyDiv(self.token_start_position, self.source_buffer.get_position(),
                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '<', '<', '=' ) => {
                for i in 1 ..= 3 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyShiftLeftAssign(self.token_start_position, self.source_buffer.get_position(),
                                                     match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '<', '<', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyShiftLeft(self.token_start_position, self.source_buffer.get_position(),
                                               match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '<', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyLessEqual(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '<', '>', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyNotEqual(self.token_start_position, self.source_buffer.get_position(),
                                               match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '<', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyLess(self.token_start_position, self.source_buffer.get_position(),
                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '>', '>', '=' ) => {
                for i in 1 ..= 3 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyShiftRightAssign(self.token_start_position, self.source_buffer.get_position(),
                                                     match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '>', '>', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyShiftRight(self.token_start_position, self.source_buffer.get_position(),
                                               match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '>', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyGreaterEqual(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '>', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyGreater(self.token_start_position, self.source_buffer.get_position(),
                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '.', '.', '.' ) => {
                for i in 1 ..= 3 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyElipsis(self.token_start_position, self.source_buffer.get_position(),
                                                     match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '.', '0' ..= '9' , _ ) => {
                let _ = self.source_buffer.advance();
                // Handle numbers starting with '.' later!

                Err("NOT YET IMPLEMENTED!".to_string())
            },
            ( '.', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyDot(self.token_start_position, self.source_buffer.get_position(),
                                              match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '+', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyPlusAssign(self.token_start_position, self.source_buffer.get_position(),
                                                   match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '+', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyPlus(self.token_start_position, self.source_buffer.get_position(),
                                              match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '-', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyMinusAssign(self.token_start_position, self.source_buffer.get_position(),
                                                 match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '-', '>', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyArrow(self.token_start_position, self.source_buffer.get_position(),
                                                 match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '-', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyMinus(self.token_start_position, self.source_buffer.get_position(),
                                           match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '%', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyModuloAssign(self.token_start_position, self.source_buffer.get_position(),
                                                 match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '%', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyModulo(self.token_start_position, self.source_buffer.get_position(),
                                           match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '@', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyMatriceAssign(self.token_start_position, self.source_buffer.get_position(),
                                                 match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '@', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyMatrice(self.token_start_position, self.source_buffer.get_position(),
                                           match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( ':', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyColonAssign(self.token_start_position, self.source_buffer.get_position(),
                                                    match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( ':', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyColon(self.token_start_position, self.source_buffer.get_position(),
                                              match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '&', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyBitAndAssign(self.token_start_position, self.source_buffer.get_position(),
                                                    match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '&', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyBitAnd(self.token_start_position, self.source_buffer.get_position(),
                                              match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '|', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyBitOrAssign(self.token_start_position, self.source_buffer.get_position(),
                                                   match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '|', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyBitOr(self.token_start_position, self.source_buffer.get_position(),
                                             match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '^', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyBitXorAssign(self.token_start_position, self.source_buffer.get_position(),
                                                  match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '^', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyBitXor(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '~', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyBitInvert(self.token_start_position, self.source_buffer.get_position(),
                                             match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( ';', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PySemiColon(self.token_start_position, self.source_buffer.get_position(),
                                             match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( ',', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyComa(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '=', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyEqual(self.token_start_position, self.source_buffer.get_position(),
                                                   match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '=', _ , _ ) => {
                let _ = self.source_buffer.advance();
                Ok(Box::new( Token::PyAssign(self.token_start_position, self.source_buffer.get_position(),
                                             match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '!', '=', _ ) => {
                for i in 1 ..= 2 { let _ = self.source_buffer.advance(); }
                Ok(Box::new( Token::PyNotEqual(self.token_start_position, self.source_buffer.get_position(),
                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '(', _ , _ ) => {
                let _ = self.source_buffer.advance();
                self.parenthesis.push(')');
                Ok(Box::new( Token::PyLeftParen(self.token_start_position, self.source_buffer.get_position(),
                                             match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '[', _ , _ ) => {
                let _ = self.source_buffer.advance();
                self.parenthesis.push(']');
                Ok(Box::new( Token::PyLeftBracket(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( '{', _ , _ ) => {
                let _ = self.source_buffer.advance();
                self.parenthesis.push('}');
                Ok(Box::new( Token::PyLeftCurly(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
            },
            ( ')', _ , _ ) => {
                let _ = self.source_buffer.advance();
                match &self.parenthesis.last() {
                    Some( ')' ) => {
                        self.parenthesis.pop();
                        Ok(Box::new( Token::PyRightParen(self.token_start_position, self.source_buffer.get_position(),
                                                        match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
                    },
                    _ => {
                        let txt = format!("Syntax Error at {} - Mismatch in parenthesis, expected ')'!", self.get_position());
                        Err(txt)
                    }
                }
            },
            ( ']', _ , _ ) => {
                let _ = self.source_buffer.advance();
                match &self.parenthesis.last() {
                    Some( ']' ) => {
                        self.parenthesis.pop();
                        Ok(Box::new( Token::PyRightBracket(self.token_start_position, self.source_buffer.get_position(),
                                                         match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
                    },
                    _ => {
                        let txt = format!("Syntax Error at {} - Mismatch in parenthesis, expected ']'!", self.get_position());
                        Err(txt)
                    }
                }
            },
            ( '}', _ , _ ) => {
                let _ = self.source_buffer.advance();
                match &self.parenthesis.last() {
                    Some( '}' ) => {
                        self.parenthesis.pop();
                        Ok(Box::new( Token::PyRightCurly(self.token_start_position, self.source_buffer.get_position(),
                                                           match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ) ))
                    },
                    _ => {
                        let txt = format!("Syntax Error at {} - Mismatch in parenthesis, expected right curly!", self.get_position());
                        Err(txt)
                    }
                }
            },
            ( s, _ , _ ) if s.is_alphabetic() || s == '_' => {
                let mut buffer = String::new();
                while self.source_buffer.get_char().is_alphanumeric() || self.source_buffer.get_char() == '_' {
                    buffer.push(self.source_buffer.get_char());
                    let _ = self.source_buffer.advance();
                }
                match buffer.as_str() {
                    "False" => Ok(Box::new(Token::PyFalse(self.token_start_position, self.source_buffer.get_position(),
                                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "None" => Ok(Box::new(Token::PyNone(self.token_start_position, self.source_buffer.get_position(),
                                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "True" => Ok(Box::new(Token::PyTrue(self.token_start_position, self.source_buffer.get_position(),
                                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "and" => Ok(Box::new(Token::PyAnd(self.token_start_position, self.source_buffer.get_position(),
                                                        match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "as" => Ok(Box::new(Token::PyAs(self.token_start_position, self.source_buffer.get_position(),
                                                      match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "assert" => Ok(Box::new(Token::PyAssert(self.token_start_position, self.source_buffer.get_position(),
                                                      match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "async" => Ok(Box::new(Token::PyAsync(self.token_start_position, self.source_buffer.get_position(),
                                                      match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "await" => Ok(Box::new(Token::PyAwait(self.token_start_position, self.source_buffer.get_position(),
                                                      match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "break" => Ok(Box::new(Token::PyBreak(self.token_start_position, self.source_buffer.get_position(),
                                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "class" => Ok(Box::new(Token::PyClass(self.token_start_position, self.source_buffer.get_position(),
                                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "continue" => Ok(Box::new(Token::PyContinue(self.token_start_position, self.source_buffer.get_position(),
                                                          match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "def" => Ok(Box::new(Token::PyDef(self.token_start_position, self.source_buffer.get_position(),
                                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "del" => Ok(Box::new(Token::PyDel(self.token_start_position, self.source_buffer.get_position(),
                                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "elif" => Ok(Box::new(Token::PyElif(self.token_start_position, self.source_buffer.get_position(),
                                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "else" => Ok(Box::new(Token::PyElse(self.token_start_position, self.source_buffer.get_position(),
                                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "except" => Ok(Box::new(Token::PyExcept(self.token_start_position, self.source_buffer.get_position(),
                                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    _ => Ok(Box::new(Token::Empty))
                }


            }


            _ => {
                let txt = format!( "Lexical error at ({}), found '{}'", self.source_buffer.get_position(), self.source_buffer.get_char() );
                Err(txt)
            }
        }

    }

    fn get_position(&self) -> u32 {
        0u32
    }

}

// UnitTests for tokenizer for Python language ////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::Token;
    use crate::Token::PySemiColon;

    #[test]
    fn tokenizer_operator_or_delimiter_shift_left_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<<=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyShiftLeftAssign( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_shift_left() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<<".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyShiftLeft( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_less_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyLessEqual( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_not_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<>".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyNotEqual( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_less() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyLess( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_shift_right_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">>=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyShiftRightAssign( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_shift_right() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">>".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyShiftRight( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_greater_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyGreaterEqual( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_greater() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyGreater( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_power_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "**=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyPowerAssign( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_power() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "**".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyPower( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_mul_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "*=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyMulAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_mul() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "*".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyMul( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_floor_div_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "//=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyFloorDivAssign( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_floor_div() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "//".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyFloorDiv( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_div_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "/=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyDivAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_div() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "/".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyDiv( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_elipsis() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "...".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyElipsis( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_dot() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyDot( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_plus_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "+=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyPlusAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_plus() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "+".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyPlus( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_minus_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "-=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyMinusAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_arrow() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "->".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyArrow( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_minus() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "-".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyMinus( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_modulo_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "%=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyModuloAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_modulo() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "%".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyModulo( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_colon_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ":=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyColonAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_colon() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ":".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyColon( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_and_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "&=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBitAndAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_and() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "&".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBitAnd( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_or_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "|=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBitOrAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_or() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "|".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBitOr( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_xor_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "^=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBitXorAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_xor() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "^".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBitXor( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_bit_matrice_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "@=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyMatriceAssign( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_matrice() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "@".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyMatrice( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "==".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyEqual( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_not_equal_default() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "!=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyNotEqual( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "=".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyAssign( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_semicolon() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ";".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PySemiColon( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_comma() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ",".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyComa( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_left_paren() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "(".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyLeftParen( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_left_bracket() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "[".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyLeftBracket( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_left_curly() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "{".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyLeftCurly( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_right_paren() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ")".to_string() ) );
        tokenizer.parenthesis.push(')');
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRightParen( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_right_paren_failing() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ")".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRightParen( 0u32, 1u32, None) => assert!(false),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(true)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_right_bracket() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "]".to_string() ) );
        tokenizer.parenthesis.push(']');
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRightBracket( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_right_bracket_failing() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "]".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRightBracket( 0u32, 1u32, None) => assert!(false),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(true)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_right_curly() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "}".to_string() ) );
        tokenizer.parenthesis.push('}');
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRightCurly( 0u32, 1u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_operator_or_delimiter_right_bracket_curly() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "}".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRightCurly( 0u32, 1u32, None) => assert!(false),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(true)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_false() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "False".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyFalse( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_none() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "None".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyNone( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_true() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "True".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyTrue( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_and() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "and".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyAnd( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_as() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "as".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyAs( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_assert() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "assert".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyAssert( 0u32, 6u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_async() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "async".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyAsync( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_await() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "await".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyAwait( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_break() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "break".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyBreak( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_class() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "class".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyClass( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_continue() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "continue".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyContinue( 0u32, 8u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_def() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "def".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyDef( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_del() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "del".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyDel( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_elif() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "elif".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyElif( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_else() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "else".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyElse( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_except() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "except".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyExcept( 0u32, 6u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }
}
