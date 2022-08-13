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
    fn handle_string(&mut self, start: u32, triple: bool, prefix: Option<String>) -> Result<Box<Token>, String>;
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
            ( '.', s , _ ) if s.is_ascii_digit() == false => {
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
                    "finally" => Ok(Box::new(Token::PyFinally(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "for" => Ok(Box::new(Token::PyFor(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "from" => Ok(Box::new(Token::PyFrom(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "global" => Ok(Box::new(Token::PyGlobal(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "if" => Ok(Box::new(Token::PyIf(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "import" => Ok(Box::new(Token::PyImport(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "in" => Ok(Box::new(Token::PyIn(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "is" => Ok(Box::new(Token::PyIs(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "lambda" => Ok(Box::new(Token::PyLambda(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "nonlocal" => Ok(Box::new(Token::PyNonLocal(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "not" => Ok(Box::new(Token::PyNot(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "or" => Ok(Box::new(Token::PyOr(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "pass" => Ok(Box::new(Token::PyPass(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "raise" => Ok(Box::new(Token::PyRaise(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "return" => Ok(Box::new(Token::PyReturn(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "try" => Ok(Box::new(Token::PyTry(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "while" => Ok(Box::new(Token::PyWhile(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "with" => Ok(Box::new(Token::PyWith(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
                    "yield" => Ok(Box::new(Token::PyYield(self.token_start_position, self.source_buffer.get_position(),
                                                            match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) } ))),
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
                    "RF" if self.source_buffer.get_char() == '\'' || self.source_buffer.get_char() == '"' => {
                        match self.source_buffer.peek_three_chars() {
                            ( '\'', '\'', '\'' ) |
                            ( '"', '"', '"' ) => {
                                self.handle_string(self.token_start_position, true, Some(buffer))
                            },
                            ( '\'', '\'', _  ) |
                            ( '"', '"', _ ) => {
                                let mut buf = String::new();
                                for i in 1 ..= 2 {  buf.push(self.source_buffer.get_char()); self.source_buffer.advance(); }
                                Ok( Box::new( Token::AtomString(self.token_start_position, self.source_buffer.get_position(),
                                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) }, Box::new(buf), Some( buffer) )) )
                            },
                            _ => self.handle_string(self.token_start_position, false, Some(buffer))
                        }
                    }
                    _ => Ok(Box::new(Token::AtomName(self.token_start_position, self.source_buffer.get_position(),
                                                     match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) }, Box::new(buffer))))
                }
            },
            ( '0' , _ , _  ) => {
                let mut buffer = String::new();
                buffer.push(self.source_buffer.get_char());
                let _ = self.source_buffer.advance();
                match self.source_buffer.get_char() {
                    'x' | 'X' => {  /* Hex digits */
                        buffer.push(self.source_buffer.get_char());
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' | '_' => {},
                            _ => {
                                return Err(format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", self.get_position()))
                            }
                        }
                        while match self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match &self.source_buffer.get_char() {
                                    '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' => {},
                                    _ => {
                                        return Err(format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", self.get_position()))
                                    }
                                }
                                true
                            },
                            '0' ..= '9' | 'a' ..= 'f' | 'A' ..= 'F' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    'o' | 'O' => {  /* Octet digits */
                        buffer.push(self.source_buffer.get_char());
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '0' ..= '7' | '_' => {},
                            _ => {
                                return Err( format!("Syntax Error at {} - Expected digit or '_' after '0o' or '0O'!", self.get_position()) )
                            }
                        }
                        while match self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '0' ..= '7' => {},
                                    _ => {
                                        return Err( format!("Syntax Error at {} - Expected digit or '_' after '0o' or '0O'!", self.get_position()) )
                                    }
                                }
                                true
                            },
                            '0' ..= '7' => {
                                buffer.push( self.source_buffer.get_char().clone() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    'b' | 'B' => {  /* Binary digits */
                        buffer.push(self.source_buffer.get_char());
                        let _ = self.source_buffer.advance();

                        match self.source_buffer.get_char() {
                            '0' | '1' | '_' => {},
                            _ => {
                                return Err(format!("Syntax Error at {} - Expected digit or '_' after '0b' or '0B'!", self.get_position()) )
                            }
                        }
                        while match self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '0' | '1' => {},
                                    _ => {
                                        return Err( format!("Syntax Error at {} - Expected digit or '_' after '0b' or '0B'!", self.get_position()) )
                                    }
                                }
                                true
                            },
                            '0' | '1' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {
                        let mut non_zero = false;
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        while   match self.source_buffer.get_char() {
                            '0' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            '1' ..= '9' => {
                                non_zero = true;
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                        match self.source_buffer.get_char() {
                            '.'  => {
                                non_zero = false;
                                buffer.push(self.source_buffer.get_char());
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '_' => return Err( format!("Syntax Error at {} - Expected digit after '.'!", &self.get_position()) ),
                                    _ => {}
                                }
                                while   match self.source_buffer.get_char() {
                                    '_' => {
                                        buffer.push( self.source_buffer.get_char() );
                                        let _ = self.source_buffer.advance();
                                        match self.source_buffer.get_char() {
                                            '0'..='9' => true,
                                            _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
                                        }
                                    },
                                    '0' ..= '9' => {
                                        buffer.push( self.source_buffer.get_char() );
                                        let _ = self.source_buffer.advance();
                                        true
                                    },
                                    _ => false
                                } {};
                            },
                            _ => {}
                        }
                        match self.source_buffer.get_char() {
                            'e' | 'E' => {
                                non_zero = false;
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '+' | '-' => {
                                        buffer.push(self.source_buffer.get_char());
                                        let _ = self.source_buffer.advance();
                                    },
                                    _ => {}
                                }
                                while   match self.source_buffer.get_char() {
                                    '_' => {
                                        buffer.push( self.source_buffer.get_char() );
                                        let _ = self.source_buffer.advance();
                                        match self.source_buffer.get_char() {
                                            '0'..='9' => true,
                                            _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
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
                        match self.source_buffer.get_char() {
                            'j' | 'J' => {
                                non_zero = false;
                                buffer.push(self.source_buffer.get_char());
                                let _ = self.source_buffer.advance();
                            },
                            _ => {}
                        }
                        match &non_zero {
                            true => return Err( format!("Syntax Error at {} - Leading zero in a integer number is not allowed'!", self.get_position()) ),
                            _ => {}
                        }
                    }
                }
                Ok( Box::new( Token::AtomNumber(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) }, Box::new(buffer)  ) ) )
            },
            ( '.', '0' ..= '9', _ ) => {
                let mut buffer = String::new();
                for i in 1..= 2 {
                    buffer.push( self.source_buffer.get_char() );
                    let _ = self.source_buffer.advance();
                }
                while   match self.source_buffer.get_char() {
                    '_' => {
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '0'..='9' => true,
                            _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
                        }
                    },
                    '0' ..= '9' => {
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        true
                    },
                    _ => false
                } {};
                match self.source_buffer.get_char() {
                    'e' | 'E' => {
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '+' | '-' => {
                                buffer.push(self.source_buffer.get_char());
                                let _ = self.source_buffer.advance();
                            },
                            _ => {}
                        }
                        while   match self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {}
                }
                match self.source_buffer.get_char() {
                    'j' | 'J' => {
                        buffer.push(self.source_buffer.get_char());
                        let _ = self.source_buffer.advance();
                    },
                    _ => {}
                }
                Ok( Box::new( Token::AtomNumber(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) }, Box::new(buffer)  ) ) )
            },
            ( '1' ..= '9', _ , _ ) => {
                let mut buffer = String::new();
                while   match self.source_buffer.get_char() {
                    '_' => {
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '0'..='9' => true,
                            _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
                        }
                    },
                    '0' ..= '9' => {
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        true
                    },
                    _ => false
                } {};
                match self.source_buffer.get_char() {
                    '.'  => {
                        buffer.push(self.source_buffer.get_char());
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '_' => return Err( format!("Syntax Error at {} - Expected digit after '.'!", &self.get_position()) ),
                            _ => {}
                        }
                        while   match self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {}
                }
                match &self.source_buffer.get_char() {
                    'e' | 'E' => {
                        buffer.push( self.source_buffer.get_char() );
                        let _ = self.source_buffer.advance();
                        match self.source_buffer.get_char() {
                            '+' | '-' => {
                                buffer.push(self.source_buffer.get_char());
                                let _ = self.source_buffer.advance();
                            },
                            _ => {}
                        }
                        while   match self.source_buffer.get_char() {
                            '_' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                match self.source_buffer.get_char() {
                                    '0'..='9' => true,
                                    _ => return Err( format!("Syntax Error at {} - Expected digit or '_' after '0x' or '0X'!", &self.get_position()) )
                                }
                            },
                            '0' ..= '9' => {
                                buffer.push( self.source_buffer.get_char() );
                                let _ = self.source_buffer.advance();
                                true
                            },
                            _ => false
                        } {};
                    },
                    _ => {}
                }
                match self.source_buffer.get_char() {
                    'j' | 'J' => {
                        buffer.push(self.source_buffer.get_char());
                        let _ = self.source_buffer.advance();
                    },
                    _ => {}
                }
                Ok( Box::new( Token::AtomNumber(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) }, Box::new(buffer)  ) ) )
            },
            ( '\'', '\'', '\'' ) |
            ( '"', '"', '"' ) => {
                self.handle_string(self.token_start_position, true, None)
            },
            ( '\'', '\'', _  ) |
            ( '"', '"', _ ) => {
                let mut buf = String::new();
                for i in 1 ..= 2 {  buf.push(self.source_buffer.get_char()); self.source_buffer.advance(); }
                Ok( Box::new( Token::AtomString(self.token_start_position, self.source_buffer.get_position(),
                                                match trivia_collector.len() { 0 => None, _ => Some( { trivia_collector.reverse(); trivia_collector } ) }, Box::new(buf), None )) )
            },
            ( '\'', _ , _  ) |
            ( '"', _ , _  )     => self.handle_string(self.token_start_position, false, None),
            _ => {
                let txt = format!( "Lexical error at ({}), found '{}'", self.source_buffer.get_position(), self.source_buffer.get_char() );
                Err(txt)
            }
        }
    }

    fn handle_string(&mut self, start: u32, triple: bool, prefix: Option<String>) -> Result<Box<Token>, String> {
        Ok(Box::new(Token::Empty))
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

    #[test]
    fn tokenizer_reserved_keyword_finally() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "finally".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyFinally( 0u32, 7u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_for() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "for".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyFor( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_from() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "from".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyFrom( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_global() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "global".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyGlobal( 0u32, 6u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_if() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "if".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyIf( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_import() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "import".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyImport( 0u32, 6u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_in() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "in".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyIn( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_is() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "is".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyIs( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_lambda() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "lambda".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyLambda( 0u32, 6u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_nonlocal() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "nonlocal".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyNonLocal( 0u32, 8u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_not() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "not".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyNot( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_or() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "or".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyOr( 0u32, 2u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_pass() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "pass".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyPass( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_raise() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "raise".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyRaise( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_return() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "return".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyReturn( 0u32, 6u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_try() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "try".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyTry( 0u32, 3u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_while() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "while".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyWhile( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_with() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "with".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyWith( 0u32, 4u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_reserved_keyword_yield() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "yield".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::PyYield( 0u32, 5u32, None) => assert!(true),
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_name_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "__init__".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomName( 0u32, 8u32, None, txt) => {
                        assert_eq!("__init__", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_name_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "rf".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomName( 0u32, 2u32, None, txt) => {
                        assert_eq!("rf", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_name_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "T3est".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomName( 0u32, 5u32, None, txt) => {
                        assert_eq!("T3est", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_hex_number_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0xaf10".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 6u32, None, txt) => {
                        assert_eq!("0xaf10", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_hex_number_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0xAF10".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 6u32, None, txt) => {
                        assert_eq!("0xAF10", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_hex_number_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0x_af_10".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 8u32, None, txt) => {
                        assert_eq!("0x_af_10", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_octet_number_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0o755".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 5u32, None, txt) => {
                        assert_eq!("0o755", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_octet_number_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0O755".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 5u32, None, txt) => {
                        assert_eq!("0O755", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_octet_number_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0O_7_5_5".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 8u32, None, txt) => {
                        assert_eq!("0O_7_5_5", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_binary_number_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0b111".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 5u32, None, txt) => {
                        assert_eq!("0b111", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_binary_number_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0B111".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 5u32, None, txt) => {
                        assert_eq!("0B111", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_binary_number_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "0b_1_0_1".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 8u32, None, txt) => {
                        assert_eq!("0b_1_0_1", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 2u32, None, txt) => {
                        assert_eq!(".0", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 6u32, None, txt) => {
                        assert_eq!(".0_0_1", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1e3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 10u32, None, txt) => {
                        assert_eq!(".0_0_1e3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_4() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1e-3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 11u32, None, txt) => {
                        assert_eq!(".0_0_1e-3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_5() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1e+3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 11u32, None, txt) => {
                        assert_eq!(".0_0_1e+3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_6() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1e+3_4j".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 12u32, None, txt) => {
                        assert_eq!(".0_0_1e+3_4j", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_7() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1E3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 10u32, None, txt) => {
                        assert_eq!(".0_0_1E3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_8() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1E-3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 11u32, None, txt) => {
                        assert_eq!(".0_0_1E-3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_9() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1E+3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 11u32, None, txt) => {
                        assert_eq!(".0_0_1E+3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_10() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0_0_1E+3_4J".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 12u32, None, txt) => {
                        assert_eq!(".0_0_1E+3_4J", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_11() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".001E+34J".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 9u32, None, txt) => {
                        assert_eq!(".001E+34J", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_12() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0J".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 3u32, None, txt) => {
                        assert_eq!(".0J", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_dot_digits__number_13() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( ".0j".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 3u32, None, txt) => {
                        assert_eq!(".0j", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 2u32, None, txt) => {
                        assert_eq!("12", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12.".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 3u32, None, txt) => {
                        assert_eq!("12.", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12e3".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 4u32, None, txt) => {
                        assert_eq!("12e3", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_4() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12e-3".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 5u32, None, txt) => {
                        assert_eq!("12e-3", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_5() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12e+3".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 5u32, None, txt) => {
                        assert_eq!("12e+3", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_6() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12e+3j".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 6u32, None, txt) => {
                        assert_eq!("12e+3j", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_7() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "12e+3J".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 6u32, None, txt) => {
                        assert_eq!("12e+3J", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_8() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "1_2e+3_4J".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 9u32, None, txt) => {
                        assert_eq!("1_2e+3_4J", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_9() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "1.2e+3_4J".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 9u32, None, txt) => {
                        assert_eq!("1.2e+3_4J", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_10() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "1.2e+3_4".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 8u32, None, txt) => {
                        assert_eq!("1.2e+3_4", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_11() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "1.2e+3".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 6u32, None, txt) => {
                        assert_eq!("1.2e+3", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_nonzero_digits_number_12() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "1.2".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomNumber( 0u32, 3u32, None, txt) => {
                        assert_eq!("1.2", *txt)
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_1() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 2u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert!(false), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_2() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "''".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 2u32, None, txt, prefix) => {
                        assert_eq!("''", *txt);
                        match prefix { Some(x) => assert!(false), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_3() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "r''".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("''", *txt);
                        match prefix { Some(x) => assert_eq!("r", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_4() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "r\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert_eq!("r", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_5() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "R\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert_eq!("R", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_6() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "u\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert_eq!("u", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_7() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "U\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert_eq!("U", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_8() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "f\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert_eq!("f", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }

    #[test]
    fn tokenizer_literal_string_empty_9() {
        let mut tokenizer = Box::new( PythonCoreTokenizer::new( "F\"\"".to_string() ) );
        match tokenizer.get_symbol() {
            Ok( s ) => {
                match *s {
                    Token::AtomString( 0u32, 3u32, None, txt, prefix) => {
                        assert_eq!("\"\"", *txt);
                        match prefix { Some(x) => assert_eq!("F", x), _ => assert!(true) }
                    },
                    _ => assert!(false)
                }
            }
            Err( e ) => assert!(false)
        }
    }
}
