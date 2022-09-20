
use crate::{ASTNode, Token };
use crate::parser::parser::{Parser, PythonCoreParser };
use crate::parser::expressions::Expressions;
use crate::parser::patterns::Patterns;
use crate::parser::tokenizer::Tokenizer;

pub trait Blocks {
    fn parse_blocks_eval_input(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_file_input(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_single_input(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_func_type_input(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_decorator(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_decorators(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_decorated(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_async_func_def(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_func_def(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_parameters(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_typed_args_list(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_tfp_def_assign(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_tfp_def(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_func_body_suite(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_blocks_class_def(&mut self) -> Result<Box<ASTNode>, String>;
}


impl Blocks for PythonCoreParser {
    fn parse_blocks_eval_input(&mut self) -> Result<Box<ASTNode>, String> {
        let _ = self.advance();
        let start_pos = self.lexer.get_position();
        let right_node = self.parse_expressions_testlist()?;
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        while
            match self.symbol.clone() {
                Ok(s) => {
                    match &*s {
                        Token::Newline(..) => {
                            separators_list.push( s );
                            let _ = self.advance();
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in eval expression!", start_pos))
            } { };
        separators_list.reverse();
        match self.symbol.clone() {
            Ok(s2) => {
                match &*s2 {
                    Token::EOF(..) => {
                        let symbol = s2;
                        Ok(Box::new( ASTNode::EvalInput(start_pos, self.lexer.get_position(), right_node, separators_list, symbol) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting send of file in eval expression!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in eval expression!", start_pos))
        }
    }

    fn parse_blocks_file_input(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_single_input(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_func_type_input(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_decorator(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_decorators(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_decorated(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_async_func_def(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_func_def(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_parameters(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_typed_args_list(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_tfp_def_assign(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_tfp_def(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_func_body_suite(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_blocks_class_def(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }
}


// UnitTests for blocks rules /////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::parser::expressions::Expressions;
    use crate::parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::parser::trivias::Trivia;
    use crate::parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn blocks_empty_template() {
        assert!(true)
    }

}
