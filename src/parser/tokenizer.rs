
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };
use crate::parser::source_buffer::{ SourceBuffer };


// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////


pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>,
    trivia_collector: Box<Vec<Box<Trivia>>>,
    symbol: Option<Box<Token>>
}


// Implementing functions related to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) ),
            trivia_collector: Box::new(Vec::new() ),
            symbol: Some( Box::new( Token::Empty ) )
        }
    }

    pub fn advance(&self) -> () {}

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
        *self.source_buffer.get_position()
    }

    /// This method checks for valid operator or delimiter including pairing parenthezis if present before returning token or Option<Token> = None.
    fn is_operator_or_delimiter(&mut self, start_pos: &u32, a: &char, b: &char, c: &char) -> Option<Token> {
        match ( &a, &b, &c ) {
            ( '*', '*', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPowerAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '*', '*', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPower(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '*', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMulAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '*', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMul(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', '/', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyFloorDivAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', '/', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyFloorDiv(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyDivAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '/', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyDiv(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '.', '.', '.' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyElipsis(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '.', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyDot(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '<', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftLeftAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '<', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftLeft(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyLessEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', '>', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyNotEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '<', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyLess(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', '>', '=' ) => {
                for i in 1 ..= 3 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftRightAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', '>', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyShiftRight(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyGreaterEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '>', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyGreater(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '+', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPlusAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '+', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyPlus(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '-', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMinusAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '-', '>', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyArrow(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '-', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMinus(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '%', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyModuloAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '%', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyModulo(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '@', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMatriceAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '@', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyMatrice(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '=', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '=', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '!', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyNotEqual(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '&', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitAndAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '&', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitAnd(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '|', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitOrAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '|', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitOr(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '^', '=', _ ) => {
                for i in 1 ..= 2 { self.source_buffer.advance() };
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitXorAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '^', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyBitXor(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },

            ( _ , _ , _ ) => {
                None
            }
        }
    }

    /// This method checks for reserved keywords or atom name literal and provides token with position and trivia collected in fron ot token
    fn is_reserved_keyword(&self, start_pos: &u32, end_pos: &u32, trivia: Option<Box<Vec<Box<Trivia>>>>, buffer: &str) -> Token {
        match &*buffer {
            "False" => Token::PyFalse(*start_pos, *end_pos, trivia),
            "None" => Token::PyNone(*start_pos, *end_pos, trivia),
            "True" => Token::PyTrue(*start_pos, *end_pos, trivia),
            "and" => Token::PyAnd(*start_pos, *end_pos, trivia),
            "as" => Token::PyAs(*start_pos, *end_pos, trivia),
            "assert" => Token::PyAssert(*start_pos, *end_pos, trivia),
            "async" => Token::PyAsync(*start_pos, *end_pos, trivia),
            "await" => Token::PyAwait(*start_pos, *end_pos, trivia),
            "break" => Token::PyBreak(*start_pos, *end_pos, trivia),
            "class" => Token::PyClass(*start_pos, *end_pos, trivia),
            "continue" => Token::PyContinue(*start_pos, *end_pos, trivia),
            "def" => Token::PyDef(*start_pos, *end_pos, trivia),
            "del" => Token::PyDel(*start_pos, *end_pos, trivia),
            "elif" => Token::PyElif(*start_pos, *end_pos, trivia),
            "else" => Token::PyElse(*start_pos, *end_pos, trivia),
            "except" => Token::PyExcept(*start_pos, *end_pos, trivia),
            "finally" => Token::PyFinally(*start_pos, *end_pos, trivia),
            "for" => Token::PyFor(*start_pos, *end_pos, trivia),
            "from" => Token::PyFrom(*start_pos, *end_pos, trivia),
            "global" => Token::PyGlobal(*start_pos, *end_pos, trivia),
            "if" => Token::PyIf(*start_pos, *end_pos, trivia),
            "import" => Token::PyImport(*start_pos, *end_pos, trivia),
            "in" => Token::PyIn(*start_pos, *end_pos, trivia),
            "is" => Token::PyIs(*start_pos, *end_pos, trivia),
            "lambda" => Token::PyLambda(*start_pos, *end_pos, trivia),
            "nonlocal" => Token::PyNonLocal(*start_pos, *end_pos, trivia),
            "not" => Token::PyNot(*start_pos, *end_pos, trivia),
            "or" => Token::PyOr(*start_pos, *end_pos, trivia),
            "pass" => Token::PyPass(*start_pos, *end_pos, trivia),
            "raise" => Token::PyRaise(*start_pos, *end_pos, trivia),
            "return" => Token::PyReturn(*start_pos, *end_pos, trivia),
            "try" => Token::PyTry(*start_pos, *end_pos, trivia),
            "while" => Token::PyWhile(*start_pos, *end_pos, trivia),
            "with" => Token::PyWith(*start_pos, *end_pos, trivia),
            "yield" => Token::PyYield(*start_pos, *end_pos, trivia),
            _ => Token::AtomName(*start_pos, *end_pos, trivia, Box::new((*buffer.to_string()).parse().unwrap()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ASTNode::PlusAssignStmt;
    use crate::parser::tokenizer::PythonCoreTokenizer;
    use crate::Token;


    #[test]
    fn operator_or_delimiter_power_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "**=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyPowerAssign( 0u32, 3u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_power() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "**".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyPower( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_mul_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "*=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyMulAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_mul() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "*".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyMul( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_floor_div_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "//=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyFloorDivAssign( 0u32, 3u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_floor_div() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "//".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyFloorDiv( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_div_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "/=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyDivAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_div() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "/".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyDiv( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_elipsis() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "...".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyElipsis( 0u32, 3u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_dot() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyDot( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_plus_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "+=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyPlusAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_plus() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "+".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyPlus( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_minus_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "-=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyMinusAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_arrow() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "->".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyArrow( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_minus() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "-".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyMinus( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_modulo_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "%=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyModuloAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_modulo() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "%".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyModulo( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_matrice_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "@=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyMatriceAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_matrice() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "@".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyMatrice( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_shift_left_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<<=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyShiftLeftAssign( 0u32, 3u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_shift_left() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<<".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyShiftLeft( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_not_equal_legacy() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<>".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyNotEqual( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_less_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyLessEqual( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_less() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "<".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyLess( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_shift_right_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">>=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyShiftRightAssign( 0u32, 3u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_shift_right() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">>".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyShiftRight( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_greater_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyGreaterEqual( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_greater() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ">".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyGreater( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "==".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyEqual( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_not_equal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "!=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyNotEqual( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyAssign( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_and_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "&=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitAndAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_and() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "&".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitAnd( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_or_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "|=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitOrAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_or() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "|".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitOr( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_xor_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "^=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitXorAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_xor() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "^".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitXor( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn reserved_keywords_false() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"False");
        match &res {
            Token::PyFalse(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_none() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"None");
        match &res {
            Token::PyNone(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_true() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"True");
        match &res {
            Token::PyTrue(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_and() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, None, &"and");
        match &res {
            Token::PyAnd(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_as() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, None, &"as");
        match &res {
            Token::PyAs(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_assert() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, None, &"assert");
        match &res {
            Token::PyAssert(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_async() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"async");
        match &res {
            Token::PyAsync(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_await() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"await");
        match &res {
            Token::PyAwait(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_break() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"break");
        match &res {
            Token::PyBreak(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_class() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"class");
        match &res {
            Token::PyClass(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_continue() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &8u32, None, &"continue");
        match &res {
            Token::PyContinue(0u32, 8u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_def() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, None, &"def");
        match &res {
            Token::PyDef(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_del() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, None, &"del");
        match &res {
            Token::PyDel(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_elif() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"elif");
        match &res {
            Token::PyElif(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_else() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"else");
        match &res {
            Token::PyElse(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_except() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, None, &"except");
        match &res {
            Token::PyExcept(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_finally() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &7u32, None, &"finally");
        match &res {
            Token::PyFinally(0u32, 7u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_for() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, None, &"for");
        match &res {
            Token::PyFor(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_from() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"from");
        match &res {
            Token::PyFrom(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_global() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, None, &"global");
        match &res {
            Token::PyGlobal(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_if() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, None, &"if");
        match &res {
            Token::PyIf(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_import() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, None, &"import");
        match &res {
            Token::PyImport(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_in() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, None, &"in");
        match &res {
            Token::PyIn(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_is() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, None, &"is");
        match &res {
            Token::PyIs(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_lambda() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, None, &"lambda");
        match &res {
            Token::PyLambda(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_nonlocal() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &8u32, None, &"nonlocal");
        match &res {
            Token::PyNonLocal(0u32, 8u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_not() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, None, &"not");
        match &res {
            Token::PyNot(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_or() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, None, &"or");
        match &res {
            Token::PyOr(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_pass() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"pass");
        match &res {
            Token::PyPass(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_raise() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"raise");
        match &res {
            Token::PyRaise(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_return() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, None, &"return");
        match &res {
            Token::PyReturn(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_try() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, None, &"try");
        match &res {
            Token::PyTry(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_while() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"while");
        match &res {
            Token::PyWhile(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_with() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, None, &"with");
        match &res {
            Token::PyWith(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_yield() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, None, &"yield");
        match &res {
            Token::PyYield(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }
}