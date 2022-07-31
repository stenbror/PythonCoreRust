
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };
use crate::parser::source_buffer::{ SourceBuffer };


// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////


pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>,
    trivia_collector: Box<Vec<Box<Trivia>>>,
    symbol: Option<Box<Token>>,
    parenthesis: Vec<char>
}


// Implementing functions related to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    pub fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) ),
            trivia_collector: Box::new(Vec::new() ),
            symbol: Some( Box::new( Token::Empty ) ),
            parenthesis: Vec::new()
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

    fn is_hex_digit(&self, ch: char) -> bool {
        match &ch {
            'a' ..= 'f' => true,
            'A' ..= 'F' => true,
            _ => ch.is_ascii_digit()
        }
    }

    fn is_octet_digit(&self, ch: char) -> bool {
        match &ch {
            '0'..='7' => true,
            _ => false
        }
    }

    fn is_binary_digit(&self, ch: char) -> bool {
        match &ch {
            '0' ..= '1' => true,
            _ => false
        }
    }

    fn handling_strings(&mut self, prefix: Option<String>, start_pos: &u32) -> Option<Token> {
        let mut buffer : String = String::new();
        let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
        match *self.source_buffer.get_char() {
            '\'' |
            '"' => {

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
                                    "fR" |
                                    "FR" |
                                    "rf" |
                                    "rF" |
                                    "Rf" |
                                    "RF" => {
                                        self.handling_strings(Some( buffer ), token_start_position)
                                    },
                                    _ => {
                                        let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                                        self.trivia_collector = Box::new(Vec::new() );
                                        Some( Token::AtomName(*token_start_position, self.get_position(), trivia, Box::new( buffer ) ))
                                    }
                                }
                            },
                            _ => {
                                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
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
            ( '(', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                &self.parenthesis.push(')');
                Some( Token::PyLeftParen(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '[', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                &self.parenthesis.push(']');
                Some( Token::PyLeftBracket(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '{', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                &self.parenthesis.push('}');
                Some( Token::PyLeftCurly(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ')', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
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
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
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
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
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
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyColonAssign(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ':', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyColon(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ';', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PySemiColon(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( ',', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
                self.trivia_collector = Box::new(Vec::new() );
                Some( Token::PyComa(*start_pos, *self.source_buffer.get_position(), trivia ) )
            },
            ( '~', _ , _ ) => {
                self.source_buffer.advance();
                let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
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
        let trivia = if self.trivia_collector.is_empty() { None } else { Some( self.trivia_collector.clone() ) };
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
    fn operator_or_delimiter_left_paren() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "(".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyLeftParen( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_left_bracket() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "[".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyLeftBracket( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_left_curly() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "{".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyLeftCurly( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_right_paren() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ")".to_string() ) );
        tokenizer.parenthesis.push(')');
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyRightParen( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_right_bracket() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "]".to_string() ) );
        tokenizer.parenthesis.push(']');
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyRightBracket( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_right_curly() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "}".to_string() ) );
        tokenizer.parenthesis.push('}');
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyRightCurly( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_colon_Assign() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ":=".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyColonAssign( 0u32, 2u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_colon() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ":".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyColon( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_semi_colon() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ";".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PySemiColon( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_coma() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ",".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyComa( 0u32, 1u32, None ) => assert!(true),
                    _ => assert!(false)
                }
            },
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn operator_or_delimiter_bit_invert() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "~".to_string() ) );
        let ( &a, &b, &c ) = &tokenizer.source_buffer.peek_three_chars();
        let res = tokenizer.is_operator_or_delimiter(&tokenizer.get_position(), &a, &b, &c);
        match &res {
            Some(s) => {
                match &s {
                    Token::PyBitInvert( 0u32, 1u32, None ) => assert!(true),
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
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"False");
        match &res.unwrap() {
            Token::PyFalse(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_none() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, &"None");
        match &res.unwrap() {
            Token::PyNone(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_true() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32,&"True");
        match &res.unwrap() {
            Token::PyTrue(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_and() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, &"and");
        match &res.unwrap() {
            Token::PyAnd(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_as() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, &"as");
        match &res.unwrap() {
            Token::PyAs(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_assert() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, &"assert");
        match &res.unwrap() {
            Token::PyAssert(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_async() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"async");
        match &res.unwrap() {
            Token::PyAsync(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_await() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"await");
        match &res.unwrap() {
            Token::PyAwait(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_break() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"break");
        match &res.unwrap() {
            Token::PyBreak(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_class() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"class");
        match &res.unwrap() {
            Token::PyClass(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_continue() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &8u32, &"continue");
        match &res.unwrap() {
            Token::PyContinue(0u32, 8u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_def() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, &"def");
        match &res.unwrap() {
            Token::PyDef(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_del() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, &"del");
        match &res.unwrap() {
            Token::PyDel(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_elif() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, &"elif");
        match &res.unwrap() {
            Token::PyElif(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_else() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, &"else");
        match &res.unwrap() {
            Token::PyElse(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_except() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, &"except");
        match &res.unwrap() {
            Token::PyExcept(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_finally() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &7u32, &"finally");
        match &res.unwrap() {
            Token::PyFinally(0u32, 7u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_for() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, &"for");
        match &res.unwrap() {
            Token::PyFor(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_from() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, &"from");
        match &res.unwrap() {
            Token::PyFrom(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_global() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, &"global");
        match &res.unwrap() {
            Token::PyGlobal(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_if() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, &"if");
        match &res.unwrap() {
            Token::PyIf(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_import() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, &"import");
        match &res.unwrap() {
            Token::PyImport(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_in() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, &"in");
        match &res.unwrap() {
            Token::PyIn(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_is() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, &"is");
        match &res.unwrap() {
            Token::PyIs(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_lambda() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, &"lambda");
        match &res.unwrap() {
            Token::PyLambda(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_nonlocal() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &8u32, &"nonlocal");
        match &res.unwrap() {
            Token::PyNonLocal(0u32, 8u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_not() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, &"not");
        match &res.unwrap() {
            Token::PyNot(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_or() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &2u32, &"or");
        match &res.unwrap() {
            Token::PyOr(0u32, 2u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_pass() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, &"pass");
        match &res.unwrap() {
            Token::PyPass(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_raise() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"raise");
        match &res.unwrap() {
            Token::PyRaise(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_return() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &6u32, &"return");
        match &res.unwrap() {
            Token::PyReturn(0u32, 6u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_try() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &3u32, &"try");
        match &res.unwrap() {
            Token::PyTry(0u32, 3u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_while() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"while");
        match &res.unwrap() {
            Token::PyWhile(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_with() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &4u32, &"with");
        match &res.unwrap() {
            Token::PyWith(0u32, 4u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_yield() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("".to_string()));
        let res = tokenizer.is_reserved_keyword(&0u32, &5u32, &"yield");
        match &res.unwrap() {
            Token::PyYield(0u32, 5u32, _) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_atom_name1() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("match".to_string()));
        let res = tokenizer.keywords_or_name_literal();
        let tst = Box::new( String::from("match") );
        match &res.unwrap() {
            Token::AtomName(0u32, 5u32, None , tst) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_atom_name2() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("__init__(".to_string()));
        let res = tokenizer.keywords_or_name_literal();
        let tst = Box::new( String::from("__init__") );
        match &res.unwrap() {
            Token::AtomName(0u32, 8u32, None , tst) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_atom_name3() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("A34".to_string()));
        let res = tokenizer.keywords_or_name_literal();
        let tst = Box::new( String::from("A34") );
        match &res.unwrap() {
            Token::AtomName(0u32, 3u32, None , tst) => assert!(true),
            _ => assert!(false)
        }
    }

    #[test]
    fn reserved_keywords_assert_outer_function() {
        let mut tokenizer = Box::new(PythonCoreTokenizer::new("assert(".to_string()));
        let res = tokenizer.keywords_or_name_literal();
        match &res.unwrap() {
            Token::PyAssert(0u32, 6u32, None ) => assert!(true),
            _ => assert!(false)
        }
    }
}