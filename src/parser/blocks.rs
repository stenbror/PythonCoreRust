
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use crate::parser::expressions::{ Expressions };
use crate::parser::statements::{ Statements };
use crate::parser::patterns::{ Patterns };


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
        let _ = &self.lexer.advance();
        let start_pos = &self.lexer.get_position();
        let right_node = self.parse_expression_test_list();
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        while  
            match &*self.lexer.get_symbol() {
                Token::Newline( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    true
                },
                _ => {
                    false
                }
            } {};
            separators_list.reverse();
            match &*self.lexer.get_symbol() {
                Token::EOF( .. ) => {
                    let symbol = self.lexer.get_symbol();
                    let _ = &self.lexer.advance();
                    let end_pos = &self.lexer.get_position();
                    Box::new( ASTNode::EvalInput(*start_pos, *end_pos, right_node, separators_list, symbol) )
                },
                _ => {
                    panic!("Syntax Error at {} - Expected EOF at end of expression!", &self.lexer.get_position())
                }
            }
    }

    fn parse_blocks_file_input(&self) -> Box<ASTNode> {
        let _ = &self.lexer.advance();
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        while  
            match &*self.lexer.get_symbol() {
                Token::EOF( .. ) => {
                    false
                },
                Token::Newline( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    true
                },
                _ => {
                    nodes_list.push( self.parse_statements_stmt() );
                    true
                }
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        match &*self.lexer.get_symbol() {
            Token::EOF( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::FileInput(*start_pos, *end_pos, nodes_list, separators_list, symbol) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected EOF at end of file input!", &self.lexer.get_position())
            }
        }
    }

    fn parse_blocks_single_input(&self) -> Box<ASTNode> {
        let _ = &self.lexer.advance();
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol()  {
            Token::Newline( .. ) => {
                let symbol = Some( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::SingleInput(*start_pos, *end_pos, None, symbol) )
            },
            Token::PyIf( .. ) |
            Token::PyWhile( .. ) |
            Token::PyFor( .. ) |
            Token::PyTry( .. ) |
            Token::PyWith( .. ) |
            Token::PyDef( .. ) |
            Token::PyClass( .. ) |
            Token::PyAsync( .. ) |
            Token::PyMatrice( .. ) => {
                let right_node = Some( self.parse_statements_compound_stmt() );
                match &*self.lexer.get_symbol() {
                    Token::Newline( .. ) => {
                        let symbol = Some( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::SingleInput(*start_pos, *end_pos, right_node, symbol) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected NEWLINE at end of compound statement input!", &self.lexer.get_position())
                    }
                }
            },
            Token::AtomName( _ , _ , _ , txt ) => {
                match &*txt.as_str() {
                    "match" => {
                        let right_node = Some( self.parse_patterns_match() );
                        match &*self.lexer.get_symbol() {
                            Token::Newline( .. ) => {
                                let symbol = Some( self.lexer.get_symbol() );
                                let _ = &self.lexer.advance();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::SingleInput(*start_pos, *end_pos, right_node, symbol) )
                            },
                            _ => {
                                panic!("Syntax Error at {} - Expected NEWLINE at end of 'match' statement input!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        let right_node = Some( self.parse_statements_simple_stmt() );
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::SingleInput(*start_pos, *end_pos, right_node, None) )
                    }
                }
            }
            _ => {
                let right_node = Some( self.parse_statements_simple_stmt() );
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::SingleInput(*start_pos, *end_pos, right_node, None) )
            }
        }
    }

    fn parse_blocks_decorator(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyMatrice( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let left_node = self.parse_statements_dotted_name();
                let mut symbol2 : Option<Box<Token>> = None;
                let mut right_node : Option<Box<ASTNode>> = None;
                let mut symbol3 : Option<Box<Token>> = None;
                match &*self.lexer.get_symbol() {
                    Token::PyLeftParen( .. ) => {
                        symbol2 = Some( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::PyRightParen( .. ) => {},
                            _ => {
                                right_node = Some( self.parse_expression_arg_list() );
                            }
                        }
                        match &*self.lexer.get_symbol() {
                            Token::PyRightParen( .. ) => {
                                symbol3 = Some( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                            },
                            _ => {
                                panic!("Syntax Error at {} - Expected ')' in decorator statement!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {}
                }
                match &*self.lexer.get_symbol() {
                    Token::Newline( .. ) => {
                        let symbol4 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::Decorator(*start_pos, *end_pos, symbol1, left_node,  symbol2, right_node, symbol3, symbol4) )
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected NWELINE after decorator statement!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected '@' in decorator statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_blocks_decorators(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_decorated(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_async_func_def(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_func_def(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_parameters(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_typed_args_list(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_tfp_def_assign(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        Box::new( ASTNode::Empty )
    }

    fn parse_blocks_tfp_def(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
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