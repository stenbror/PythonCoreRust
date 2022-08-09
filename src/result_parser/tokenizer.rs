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
}
