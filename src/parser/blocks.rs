
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use crate::parser::patterns::{ Patterns };
use crate::parser::expressions::{ Expressions };
use crate::parser::statements::{ Statements };
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
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyClass( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::AtomName( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let mut symbol3 : Option<Box<Token>> = None;
                        let mut symbol4 : Option<Box<Token>> = None;
                        let mut left_node : Option<Box<ASTNode>> = None;
                        match &*self.lexer.get_symbol() {
                            Token::PyLeftParen( .. ) => {
                                symbol3 = Some( self.lexer.get_symbol() );
                                let _ = &self.lexer.advance();
                                match &*self.lexer.get_symbol() {
                                    Token::PyRightParen( .. ) => {},
                                    _ => {
                                        left_node = Some( self.parse_expression_var_args_list() );
                                    }
                                }
                                match &*self.lexer.get_symbol() {
                                    Token::PyRightParen( .. ) => {
                                        symbol4 = Some( self.lexer.get_symbol() );
                                        let _ = &self.lexer.advance();
                                    },
                                    _ => {
                                        panic!("Syntax Error at {} - Expected ')' in class statement!", &self.lexer.get_position())
                                    }
                                }
                            },
                            _ => {}
                        }
                        match &*self.lexer.get_symbol() {
                            Token::PyColon( .. ) => {
                                let symbol5 = self.lexer.get_symbol();
                                let _ = &self.lexer.advance();
                                let right_node = self.parse_statements_suite();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::ClassDef(*start_pos, *end_pos, symbol1, symbol2, symbol3, left_node, symbol4, symbol5, right_node) )     
                            },
                            _ => {
                                panic!("Syntax Error at {} - Expected ':' in class statement!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected Name for class statement!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'class' in statement!", &self.lexer.get_position())
            }
        }
    }
}