
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use crate::parser::patterns::{ Patterns };
use crate::parser::expressions::{ Expressions };
use std::vec;


pub trait Blocks {
    fn parse_blocks_eval_input(&self) -> Box<ASTNode>;
    fn parse_blocks_file_input(&self) -> Box<ASTNode>;
    fn parse_blocks_single_input(&self) -> Box<ASTNode>;
    fn parse_blocks_decorator(&self) -> Box<ASTNode>;
    fn parse_blocks_decorators(&self) -> Box<ASTNode>;
    fn parse_blocks_decorated(&self) -> Box<ASTNode>;
    fn parse_blocks_async_func_def(&self) -> Box<ASTNode>;
    fn parse_blocks_func_def(&self) -> Box<ASTNode>;
    fn parse_blocks_parameters(&self) -> Box<ASTNode>;
    fn parse_blocks_typed_args_list(&self) -> Box<ASTNode>;
    fn parse_blocks_tfp_def_assign(&self) -> Box<ASTNode>;
    fn parse_blocks_tfp_def(&self) -> Box<ASTNode>;
    fn parse_blocks_class_def(&self) -> Box<ASTNode>;
}


impl Blocks for PythonCoreParser {
    fn parse_blocks_eval_input(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_file_input(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_single_input(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_decorator(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_decorators(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_decorated(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_async_func_def(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_func_def(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_parameters(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_typed_args_list(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_tfp_def_assign(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_tfp_def(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_class_def(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
}