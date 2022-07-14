
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };

trait Functions {
    fn parse_functions_func_type(&self) -> Box<ASTNode>;
    fn parse_functions_type_list(&self) -> Box<ASTNode>;
}

impl Functions for PythonCoreParser {
    fn parse_functions_func_type(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_functions_type_list(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
}