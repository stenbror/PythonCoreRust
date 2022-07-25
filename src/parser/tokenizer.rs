
use crate::parser::trivias::{ Trivia };
use crate::parser::tokens::{ Token };


// Defining data structure and traits for tokenizing of PythonCore ////////////////////////////////


pub struct PythonCoreTokenizer {

}


// Implementing functions releated to tokenizing of PythonCore ////////////////////////////////////

impl PythonCoreTokenizer {
    fn new() -> PythonCoreTokenizer {
        PythonCoreTokenizer {

        }
    }

    pub fn advance(&self) -> () {}

    pub fn get_symbol(&self) -> Box<Token> {
        Box::new(Token::Empty)
    }

    pub fn get_position(&self) -> u32 {
        0
    }

    /// This method checks for reserved keywords or atom name literal and provides token with position and trivia collected in fron ot token
    fn is_reserved_keyword(start_pos: &u32, end_pos: &u32, trivia: Option<Box<Vec<Box<Trivia>>>>, buffer: &str) -> Token {
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
