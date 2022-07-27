
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };
use crate::parser::source_buffer::{ SourceBuffer };


// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////


pub struct PythonCoreTokenizer {
    source_buffer: Box<SourceBuffer>,
    trivia_collector: Box<Vec<Box<Trivia>>>,
    current_trivia: Box<Vec<Box<&'static Trivia>>>
}


// Implementing functions releated to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    fn new(buffer: String) -> PythonCoreTokenizer {
        PythonCoreTokenizer {
            source_buffer: Box::new( SourceBuffer::new(buffer) ),
            trivia_collector: Box::new(Vec::new() ),
            current_trivia: Box::new(Vec::new() )
        }
    }

    pub fn advance(&self) -> () {}

    pub fn get_symbol(&self) -> Box<Token> {
        Box::new(Token::Empty)
    }

    pub fn get_position(&self) -> u32 {
        *self.source_buffer.get_position()
    }

    /// This method checks for valid operator or delimiter including pairing parenthezis if present before returning token or Option<Token> = None.
    fn is_operator_or_delimiter(&mut self, start_pos: &u32, a: &char, b: &char, c: &char) -> Option<Token> {
        match ( &a, &b, &c ) {
            ( '*', '*', '=' ) => {
                for i in 1 .. 3 { self.source_buffer.advance() };
                let local_trivia = self.current_trivia.clone();
                self.current_trivia = Box::new( Vec::new() );
                Some( Token::PyPowerAssign(*start_pos, *self.source_buffer.get_position(), Some( local_trivia ) ) )
            },
            ( '*', '*', _ ) => {
                for i in 1 .. 2 { self.source_buffer.advance() };
                let local_trivia = self.current_trivia.clone();
                self.current_trivia = Box::new( Vec::new() );
                Some( Token::PyPower(*start_pos, *self.source_buffer.get_position(), Some( local_trivia ) ) )
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
