
use crate::{ASTNode, Statements, Token};
use crate::parser::parser::{Parser, PythonCoreParser };
use crate::parser::expressions::Expressions;
use crate::parser::functions::Functions;
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
        let _ = self.advance();
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        while
            match self.symbol.clone() {
                Ok(s) => {
                    match &*s {
                        Token::EOF(..) => false,
                        Token::Newline(..) => {
                            separators_list.push( s );
                            let _ = self.advance();
                            true
                        },
                        _ => {
                            nodes_list.push( self.parse_statements_stmt()? );
                            true
                        }
                    }
                },
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in eval expression!", start_pos))
            } { };
        separators_list.reverse();
        nodes_list.reverse();
        match self.symbol.clone() {
            Ok(s2) => {
                match &*s2 {
                    Token::EOF(..) => {
                        let symbol = s2;
                        Ok(Box::new( ASTNode::FileInput(start_pos, self.lexer.get_position(), nodes_list, separators_list, symbol) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting end of file in file input!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in file input!", start_pos))
        }
    }

    fn parse_blocks_single_input(&mut self) -> Result<Box<ASTNode>, String> {
        let _ = self.advance();
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::Newline( .. ) => {
                        let symbol = Some( s );
                        let _ = self.advance();
                        Ok(Box::new( ASTNode::SingleInput(start_pos, self.lexer.get_position(), None, symbol) ))
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
                        let right_node = Some(self.parse_statements_compound_stmt()?);
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::Newline(..) => {
                                        let symbol = Some( s2 );
                                        Ok(Box::new( ASTNode::SingleInput(start_pos, self.lexer.get_position(), right_node, symbol) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting Newline after compound statement in single input!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in single input!", start_pos))
                        }
                    },
                    Token::AtomName( _ , _ , _ , txt) => {
                        match &*txt.as_str() {
                            "match" => {
                                let right_node = Some( self.parse_patterns_match()? );
                                match self.symbol.clone() {
                                    Ok(s3) => {
                                        match &*s3 {
                                            Token::Newline(..) => {
                                                let symbol = Some( s3 );
                                                Ok(Box::new( ASTNode::SingleInput(start_pos, self.lexer.get_position(), right_node, symbol) ))
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting Newline after compound statement in single input!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting symbol in single input!", start_pos))
                                }
                            },
                            _ => {
                                let right_node = Some( self.parse_statements_simple_stmt()? );
                                Ok( Box::new( ASTNode::SingleInput(start_pos, self.lexer.get_position(), right_node, None) ) )
                            }
                        }
                    },
                    _ => {
                        let right_node = Some( self.parse_statements_simple_stmt()? );
                        Ok(Box::new( ASTNode::SingleInput(start_pos, self.lexer.get_position(), right_node, None) ))
                    }
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in single input!", start_pos))
        }
    }

    fn parse_blocks_func_type_input(&mut self) -> Result<Box<ASTNode>, String> {
        let _ = self.advance();
        let start_pos = self.lexer.get_position();
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let right_node = self.parse_functions_func_type()?;
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
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in functional input!", start_pos))
            } { };
        separators_list.reverse();
        match self.symbol.clone() {
            Ok(s2) => {
                match &*s2 {
                    Token::EOF(..) => {
                        let symbol = s2;
                        Ok( Box::new( ASTNode::FuncTypeInput(start_pos, self.lexer.get_position(), right_node, separators_list, symbol) ) )
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting end of file in function input!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in functional input!", start_pos))
        }
    }

    fn parse_blocks_decorator(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyMatrice(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let left_node = self.parse_statements_dotted_name()?;
                        let mut symbol2 : Option<Box<Token>> = None;
                        let mut right_node : Option<Box<ASTNode>> = None;
                        let mut symbol3 : Option<Box<Token>> = None;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyLeftParen(..) => {
                                        symbol2 = Some( s2 );
                                        let _ = self.advance();
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyRightParen(..) => {},
                                                    _ => {
                                                        right_node = Some( self.parse_expressions_arglist()? );
                                                    }
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in functional input!", start_pos))
                                        }
                                        match self.symbol.clone() {
                                            Ok(s4) => {
                                                match &*s4 {
                                                    Token::PyRightParen(..) => {
                                                        symbol3 = Some( s4 );
                                                        let _ = self.advance();
                                                    },
                                                    _ =>  return Err(format!("SyntaxError at {}: Expecting ')' in functional input!", start_pos))
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in functional input!", start_pos))
                                        }

                                    },
                                    _ => { }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in functional input!", start_pos))
                        }
                        match self.symbol.clone() {
                            Ok(s5) => {
                                match &*s5 {
                                    Token::Newline(..) => {
                                        let symbol4 = s5;
                                        Ok(Box::new( ASTNode::Decorator(start_pos, self.lexer.get_position(), symbol1, left_node,  symbol2, right_node, symbol3, symbol4) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting end of file in function input!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in functional input!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting '@' in decorator statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in decorator statement!", start_pos))
        }
    }

    fn parse_blocks_decorators(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_blocks_decorator()? );
        while
            match self.symbol.clone() {
                Ok(s) => {
                    match &*s {
                        Token::PyMatrice(..) => {
                            nodes_list.push( self.parse_blocks_decorator()? );
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in decorator statement!", start_pos))
            } { };
        nodes_list.reverse();
        Ok( Box::new( ASTNode::Decorators(start_pos, self.lexer.get_position(), nodes_list) ))
    }

    fn parse_blocks_decorated(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyMatrice(..) => {
                        let left_node = self.parse_blocks_decorators()?;
                        match self.symbol.clone() {
                            Ok(s) => {
                                match &*s {
                                    Token::PyClass(..) => {
                                        let right_node = self.parse_blocks_class_def()?;
                                        Ok(Box::new( ASTNode::Decorated(start_pos, self.lexer.get_position(), left_node, right_node) ))
                                    },
                                    Token::PyDef(..) => {
                                        let right_node = self.parse_blocks_func_def()?;
                                        Ok(Box::new( ASTNode::Decorated(start_pos, self.lexer.get_position(), left_node, right_node) ))
                                    },
                                    Token::PyAsync(..) => {
                                        let right_node = self.parse_blocks_async_func_def()?;
                                        Ok(Box::new( ASTNode::Decorated(start_pos, self.lexer.get_position(), left_node, right_node) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting 'class', 'def' or 'async' after '@' in decorator statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in decorator statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting '@' in decorator statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in decorator statement!", start_pos))
        }
    }

    fn parse_blocks_async_func_def(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyAsync(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_blocks_func_def()?;
                        Ok(Box::new( ASTNode::AsyncStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'async' in async statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in async statement!", start_pos))
        }
    }

    fn parse_blocks_func_def(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyDef(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::AtomName(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let left_node : Option<Box<ASTNode>> = match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyLeftParen(..) => {
                                                        Some(self.parse_blocks_parameters()?)
                                                    },
                                                    _ => None
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting 'def' in function statement!", start_pos))
                                        };
                                        let ret_node : Option<Box<( Box<Token>, Box<ASTNode> )>> = match self.symbol.clone() {
                                            Ok(s4) => {
                                                match &*s4 {
                                                    Token::PyArrow(..) => {
                                                        let symbol3 = s4;
                                                        let _ = self.advance();
                                                        let res_node = self.parse_expressions_test()?;
                                                        Some( Box::new( ( symbol3 , res_node )))
                                                    },
                                                    _ => None
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting 'def' in function statement!", start_pos))
                                        };
                                        match self.symbol.clone() {
                                            Ok(s5) => {
                                                match &*s5 {
                                                    Token::PyColon(..) => {
                                                        let symbol4 = s5;
                                                        let _ = self.advance();
                                                        let tc_symbol : Option<Box<Token>> = match self.symbol.clone() {
                                                            Ok(s6) => {
                                                                match &*s6 {
                                                                    Token::TypeComment(..) => {
                                                                        let symbol5 = s6;
                                                                        let _ = self.advance();
                                                                        Some(symbol5)
                                                                    },
                                                                    _ => None
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in function statement!", start_pos))
                                                        };
                                                        let body_node = self.parse_blocks_func_body_suite()?;
                                                        Ok(Box::new( ASTNode::FuncDef(start_pos, self.lexer.get_position(), symbol1, symbol2, left_node, ret_node, symbol4, tc_symbol, body_node ) ))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in function statement!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in function statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting literal name in function statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting 'def' in function statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'def' in function statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in function statement!", start_pos))
        }
    }

    fn parse_blocks_parameters(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyLeftParen(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let right_node : Option<Box<ASTNode>> = match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyRightParen(..) => None,
                                    _ => Some(self.parse_blocks_typed_args_list()?)
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in parameters of function statement!", start_pos))
                        };
                        match self.symbol.clone() {
                            Ok(s3) => {
                                match &*s3 {
                                    Token::PyRightParen(..) => {
                                        let symbol2 = s3;
                                        let _ = self.advance();
                                        Ok(Box::new(ASTNode::Parameter(start_pos, self.lexer.get_position(), symbol1, right_node, symbol2) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ')' in parameters of function statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in parameters of function statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting '(' in parameters of function statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in parameters of function statement!", start_pos))
        }
    }

    fn parse_blocks_typed_args_list(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let a : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let b : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let c : Box<Vec<Box<Token>>> = Box::new(Vec::new());

        // Missing rule here!

        Ok(Box::new(ASTNode::TypedArgsList(start_pos, self.lexer.get_position(), a, b, c, None, None )))
    }

    fn parse_blocks_tfp_def_assign(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left_node = self.parse_blocks_tfp_def()?;
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyAssign(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_expressions_test()?;
                        Ok(Box::new( ASTNode::TFPAssign(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                    },
                    _ => Ok( left_node )
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in parameter of function statement!", start_pos))
        }
    }

    fn parse_blocks_tfp_def(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::AtomName(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_test()?;
                                        Ok(Box::new( ASTNode::TFPDef(start_pos, self.lexer.get_position(), symbol1, Some( Box::new( ( symbol2, right_node ) ) )) ))
                                    },
                                    _ => {
                                        Ok(Box::new( ASTNode::TFPDef(start_pos, self.lexer.get_position(), symbol1, None) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in parameter of function statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting name literal in parameter of function statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in parameter of function statement!", start_pos))
        }
    }

    fn parse_blocks_func_body_suite(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::Newline(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let mut tc_symbol : Option<Box<Token>> = None;
                        let mut tc_newline : Option<Box<Token>> = None;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::TypeComment(..) => {
                                        tc_symbol = Some( s2 );
                                        let _ = self.advance();
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::Newline(..) => {
                                                        tc_newline = Some( s3 );
                                                        let _ = self.advance();
                                                    },
                                                    _ => return Err(format!("SyntaxError at {}: Expecting Newline after TypeComment in function body!", start_pos))
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in function body!", start_pos))
                                        }
                                    },
                                    _ => { }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in function body!", start_pos))
                        }
                        match self.symbol.clone() {
                            Ok(s4) => {
                                match &*s4{
                                    Token::Indent(..) => {
                                        let symbol2 = s4;
                                        let _ = self.advance();
                                        nodes_list.push(self.parse_statements_stmt()?);
                                        while
                                            match self.symbol.clone() {
                                                Ok(s5) => {
                                                    match &*s5 {
                                                        Token::Dedent(..) => false,
                                                        _ => {
                                                            nodes_list.push(self.parse_statements_stmt()?);
                                                            true
                                                        }
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in function body!", start_pos))
                                            } { };
                                        nodes_list.reverse();
                                        match self.symbol.clone() {
                                            Ok(s6) => {
                                                match &*s6 {
                                                    Token::Dedent(..) => {
                                                        let symbol3 = s6;
                                                        let _ = self.advance();
                                                        Ok(Box::new(ASTNode::FuncBodySuite(start_pos, self.lexer.get_position(), symbol1, tc_symbol, tc_newline, symbol2, nodes_list, symbol3)))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting end of block in function body!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in function body!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting indentation in function body!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in function body!", start_pos))
                        }
                    },
                    _ => self.parse_statements_simple_stmt()
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in function body!", start_pos))
        }
    }

    fn parse_blocks_class_def(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyClass(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::AtomName(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let mut symbol3 : Option<Box<Token>> = None;
                                        let mut symbol4 : Option<Box<Token>> = None;
                                        let mut left_node : Option<Box<ASTNode>> = None;
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyLeftParen(..) => {
                                                        symbol3 = Some( s3 );
                                                        let _ = self.advance();
                                                        left_node = match self.symbol.clone() {
                                                            Ok(s5) => {
                                                                match &*s5 {
                                                                    Token::PyRightParen(..) => None,
                                                                    _ => Some(self.parse_expressions_var_args_list()?)
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in class statement!", start_pos))
                                                        };
                                                        match self.symbol.clone() {
                                                            Ok(s6) => {
                                                                match &*s6 {
                                                                    Token::PyRightParen(..) => {
                                                                        symbol4 = Some( s6 );
                                                                        let _ = self.advance();
                                                                    },
                                                                    _ => return Err(format!("SyntaxError at {}: Expecting ')' in class statement!", start_pos))
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in class statement!", start_pos))
                                                        }
                                                    },
                                                    _ => { }
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in class statement!", start_pos))
                                        }
                                        match self.symbol.clone() {
                                            Ok(s4) => {
                                                match &*s4 {
                                                    Token::PyColon(..) => {
                                                        let symbol5 = s4;
                                                        let _ = self.advance();
                                                        let right_node = self.parse_statements_suite()?;
                                                        Ok(Box::new( ASTNode::ClassDef(start_pos, self.lexer.get_position(), symbol1, symbol2, symbol3, left_node, symbol4, symbol5, right_node) ) )
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in class statement!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in class statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting literal name in class statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in class statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'class' in class statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in class statement!", start_pos))
        }
    }
}


// UnitTests for blocks rules /////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {



    #[test]
    fn blocks_empty_template() {
        assert!(true)
    }

}
