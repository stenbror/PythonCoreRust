
use crate::result_parser::nodes::{ ASTNode };
use crate::result_parser::tokens::{ Token };
use crate::result_parser::parser::{ PythonCoreParser };
use crate::result_parser::expressions::{ Expressions };


pub trait Functions {
    fn parse_functions_func_type(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_functions_type_list(&mut self) -> Result<Box<ASTNode>, String>;
}


impl Functions for PythonCoreParser {
    fn parse_functions_func_type(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_functions_type_list(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }
}


// UnitTests for functions rules //////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::result_parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn statements_empty_template() {
        assert!(true)
    }

}
