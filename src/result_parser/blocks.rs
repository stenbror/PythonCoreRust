
use crate::{ASTNode, Token };
use crate::result_parser::parser::{ Parser, PythonCoreParser };
use crate::result_parser::expressions::Expressions;
use crate::result_parser::patterns::Patterns;
use crate::result_parser::tokenizer::Tokenizer;

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
        todo!()
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
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::result_parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn blocks_empty_template() {
        assert!(true)
    }

}
