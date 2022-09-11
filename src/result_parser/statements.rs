
use crate::{ASTNode, Token };
use crate::result_parser::parser::{ Parser, PythonCoreParser };
use crate::result_parser::tokenizer::Tokenizer;


pub trait Statements {

}


impl Statements for PythonCoreParser {

}


// UnitTests for statements rules /////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn statements_empty_template() {
        assert!(true)
    }

}