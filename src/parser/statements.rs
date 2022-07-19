
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };
use crate::parser::patterns::{ Patterns };
use crate::parser::expressions::{ Expressions };
use std::vec;


pub trait Statements {
    fn parse_statements_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_simple_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_small_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_expr_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_ann_assign(&self) -> Box<ASTNode>;
    fn parse_statements_del_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_pass_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_flow_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_break_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_continue_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_return_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_yield_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_raise_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_import_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_import_name(&self) -> Box<ASTNode>;
    fn parse_statements_import_from(&self) -> Box<ASTNode>;
    fn parse_statements_import_as_name(&self) -> Box<ASTNode>;
    fn parse_statements_dotted_as_name(&self) -> Box<ASTNode>;
    fn parse_statements_import_as_names(&self) -> Box<ASTNode>;
    fn parse_statements_dotted_as_names(&self) -> Box<ASTNode>;
    fn parse_statements_dotted_name(&self) -> Box<ASTNode>;
    fn parse_statements_global_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_nonlocal_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_assert_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_compound_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_async_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_if_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_elif_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_else_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_while_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_for_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_try_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_finally_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_with_stmt(&self) -> Box<ASTNode>;
    fn parse_statements_with_item(&self) -> Box<ASTNode>;
    fn parse_statements_except_clause(&self) -> Box<ASTNode>;
    fn parse_statements_suite(&self) -> Box<ASTNode>;
}


impl Statements for PythonCoreParser {
    fn parse_statements_stmt(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyIf( .. ) |
            Token::PyFor( .. ) |
            Token::PyWhile( .. ) |
            Token::PyWith( .. ) |
            Token::PyTry( .. ) |
            Token::PyAsync( .. ) |
            Token::PyMatrice( .. ) |
            Token::PyDef( .. ) |
            Token::PyClass( .. ) => { 
                self.parse_statements_compound_stmt()
            },
            Token::AtomName( _ , _ , _ , txt ) => {
                match &*txt.as_str() {
                    "match" => {
                        self.parse_patterns_match()
                    },
                    _ => {
                        self.parse_statements_simple_stmt()
                    }
                }
            },
            _ => {
                self.parse_statements_simple_stmt()
            }
        }
    }

    fn parse_statements_simple_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_statements_small_stmt() );
        while
            match &*self.lexer.get_symbol() {
                Token::PySemiColon( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::Newline( .. ) |
                        Token::EOF( .. ) => {
                            false
                        },
                        _ => {
                            nodes_list.push( self.parse_statements_small_stmt() );
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        let mut end_marker : Box<Token> = Box::new( Token::Empty );
        match &*self.lexer.get_symbol() {
            Token::Newline( .. ) => {
                end_marker = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
            },
            _ => {
                panic!("Syntax Error at {} - Expected NEWLINE after statements!", &self.lexer.get_position())
            }
        }
        let end_pos = &self.lexer.get_position();
        nodes_list.reverse();
        separators_list.reverse();
        Box::new( ASTNode::SimpleStmtList(*start_pos, *end_pos, nodes_list, separators_list, end_marker) )
    }

    fn parse_statements_small_stmt(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyDel( .. ) => {
                self.parse_statements_del_stmt()
            },
            Token::PyPass( .. ) => {
                self.parse_statements_pass_stmt()
            },
            Token::PyBreak( .. ) |
            Token::PyContinue( .. ) |
            Token::PyReturn( .. ) |
            Token::PyRaise( .. ) |
            Token::PyYield( .. ) => {
                self.parse_statements_flow_stmt()
            },
            Token::PyImport( .. ) |
            Token::PyFrom( .. ) => {
                self.parse_statements_import_stmt()
            },
            Token::PyGlobal( .. ) => {
                self.parse_statements_global_stmt()
            },
            Token::PyNonLocal( .. ) => {
                self.parse_statements_nonlocal_stmt()
            },
            Token::PyAssert( .. ) => {
                self.parse_statements_assert_stmt()
            },
            _ => {
                self.parse_statements_expr_stmt()
            }
        }
    }

    fn parse_statements_expr_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_ann_assign(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
    
    fn parse_statements_del_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyDel( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_expr_list();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::DelStmt(*start_pos, *end_pos, symbol, right_node) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'del' in del statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_pass_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyPass( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::PassStmt(*start_pos, *end_pos, symbol) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'pass' in pass statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_flow_stmt(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyBreak( .. ) => {
                self.parse_statements_break_stmt()
            },
            Token::PyContinue( .. ) => {
                self.parse_statements_continue_stmt()
            },
            Token::PyReturn( .. ) => {
                self.parse_statements_return_stmt()
            },
            Token::PyRaise( .. ) => {
                self.parse_statements_raise_stmt()
            },
            Token::PyYield( .. ) => {
                self.parse_statements_yield_stmt()
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'break', 'continue', 'return', 'raise' or 'yield' in flow statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_break_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyBreak( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::BreakStmt(*start_pos, *end_pos, symbol) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'break' in break statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_continue_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyContinue( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::ContinueStmt(*start_pos, *end_pos, symbol) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'continue' in continue statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_return_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyReturn( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PySemiColon( .. ) |
                    Token::Newline( .. ) |
                    Token::EOF( .. ) => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ReturnStmt(*start_pos, *end_pos, symbol, None) )
                    },
                    _ => {
                        let right_node = Some( self.parse_expression_testlist_star_expr() );
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ReturnStmt(*start_pos, *end_pos, symbol, right_node) )
                    }
                }    
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'return' in return statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_yield_stmt(&self) -> Box<ASTNode> {
        self.parse_expression_yield_expr()
    }

    fn parse_statements_raise_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_import_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_import_name(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_import_from(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_import_as_name(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_dotted_as_name(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_import_as_names(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_dotted_as_names(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_dotted_name(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_global_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_nonlocal_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_assert_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_compound_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_async_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_if_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_elif_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_else_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_while_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_for_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_try_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_finally_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_with_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_with_item(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_except_clause(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_suite(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
}