
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
    fn parse_statements_ann_assign(&self, start_pos: &u32, left_node: Box<ASTNode>) -> Box<ASTNode>;
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
        let start_pos = &self.lexer.get_position();
        let left_node = self.parse_expression_testlist_star_expr();
        match &*self.lexer.get_symbol() {
            Token::PyColon( .. ) => {
                self.parse_statements_ann_assign(start_pos, left_node)
            },
            Token::PyAssign( .. ) => {
                let mut nodes_list : Box<Vec<Box< ( Box<Token>, Box<ASTNode> ) >>> = Box::new(Vec::new());
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyAssign( .. ) => {
                            let ass_symbol = self.lexer.get_symbol();
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::PyYield( .. ) => {
                                    let right_node = self.parse_expression_yield_expr();
                                    nodes_list.push( Box::new( ( ass_symbol, right_node ) ) )
                                },
                                _ => {
                                    let right_node = self.parse_expression_testlist_star_expr();
                                    nodes_list.push( Box::new( ( ass_symbol, right_node ) ) )
                                }
                            }
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                nodes_list.reverse();
                match &*self.lexer.get_symbol() {
                    Token::TypeComment( .. ) => {
                        let tc_symbol = Some( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AssignmentStmt(*start_pos, *end_pos, left_node, nodes_list, tc_symbol) )
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AssignmentStmt(*start_pos, *end_pos, left_node, nodes_list, None) )
                    }
                }
            },
            Token::PyPlusAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::PlusAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::PlusAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyMinusAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::MinusAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::MinusAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyMulAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::MulAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::MulAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyPowerAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::PowerAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::PowerAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyDivAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::DivAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::DivAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyFloorDivAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::FloorDivAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::FloorDivAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyModuloAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ModuloAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ModuloAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyMatriceAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::MatriceAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::MatriceAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyBitAndAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::BitAndAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::BitAndAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyBitOrAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::BitOrAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::BitOrAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyBitXorAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::BitXorAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::BitXorAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyShiftLeftAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ShiftLeftAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ShiftLeftAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            Token::PyShiftRightAssign( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyYield( .. ) => {
                        let right_node = self.parse_expression_yield_expr();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ShiftRightAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    },
                    _ => {
                        let right_node = self.parse_expression_test_list();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ShiftRightAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node) )
                    }
                }
            },
            _ => {
                left_node
            }
        }
    }

    fn parse_statements_ann_assign(&self, start_pos: &u32, left_node: Box<ASTNode>) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyColon( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_expression_test();
                match &*self.lexer.get_symbol() {
                    Token::PyAssign( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();

                        match &*self.lexer.get_symbol() {
                            Token::PyYield( .. ) => {
                                let next_node = self.parse_expression_yield_expr();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::AnnAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node, Some( (symbol2, next_node) )) )
                            },
                            _ => {
                                let next_node = self.parse_expression_testlist_star_expr();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::AnnAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node, Some( (symbol2, next_node) )) )
                            }
                        }
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AnnAssignStmt(*start_pos, *end_pos, left_node, symbol, right_node, None) )
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected ':' in annotated assignment statement!", &self.lexer.get_position())
            }
        }
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
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyRaise( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PySemiColon( .. ) |
                    Token::Newline( .. ) |
                    Token::EOF( .. ) => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::RaiseStmt(*start_pos, *end_pos, symbol, None) )
                    },
                    _ => {
                        let left_node = self.parse_expression_test();
                        match &*self.lexer.get_symbol() {
                            Token::PyFrom( .. ) => {
                                let symbol2 = self.lexer.get_symbol();
                                let _ = &self.lexer.advance();
                                let right_node = self.parse_expression_test();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::RaiseStmt(*start_pos, *end_pos, symbol, Some( (left_node, Some( (symbol2, right_node) )) )) )
                            },
                            _ => {
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::RaiseStmt(*start_pos, *end_pos, symbol, Some( (left_node, None) )) )
                            }
                        }
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'raise' in raise statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_import_stmt(&self) -> Box<ASTNode> {
        match &*self.lexer.get_symbol() {
            Token::PyImport( .. ) => {
                self.parse_statements_import_name()
            },
            Token::PyFrom( .. ) => {
                self.parse_statements_import_from()
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'import' or 'from' in import statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_import_name(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyImport( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let right_node = self.parse_statements_dotted_as_names();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::ImportNameStmt(*start_pos, *end_pos, symbol, right_node) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'import' in import statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_import_from(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyFrom( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyDot( .. ) |
                        Token::PyElipsis( .. ) => {
                            nodes_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                nodes_list.reverse();
                let mut left_node : Option<Box<ASTNode>> = None;
                match ( nodes_list.len(), &*self.lexer.get_symbol() ) {
                    ( 0, Token::PyImport( .. ) ) => {
                        panic!("Syntax Error at {} - Expected dot(s) or 'from part' in import statement!", &self.lexer.get_position())
                    },
                    ( _ , Token::PyImport( .. ) ) => {}
                    _ => {
                        left_node = Some( self.parse_statements_dotted_name() );
                    }
                }
                match &*self.lexer.get_symbol() {
                    Token::PyImport( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::PyMul( .. ) => {
                                let symbol3 = Some( self.lexer.get_symbol() );
                                let _ = &self.lexer.advance();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::ImportFromStmt(*start_pos, *end_pos, symbol1, nodes_list, left_node, symbol2, symbol3, None, None) )
                            },
                            Token::PyLeftParen( .. ) => {
                                let symbol3 = Some( self.lexer.get_symbol() );
                                let _ = &self.lexer.advance();
                                let right_node = Some( self.parse_statements_import_as_names() );
                                match &*self.lexer.get_symbol() {
                                    Token::PyRightParen( .. ) => {
                                        let symbol4 = Some( self.lexer.get_symbol() );
                                        let _ = &self.lexer.advance();
                                        let end_pos = &self.lexer.get_position();
                                        Box::new( ASTNode::ImportFromStmt(*start_pos, *end_pos, symbol1, nodes_list, left_node, symbol2, symbol3, right_node, symbol4) )
                                },
                                    _ => {
                                        panic!("Syntax Error at {} - Expected ')' in import statement!", &self.lexer.get_position())
                                    }
                                }
                            },
                            _ => {
                                let right_node = Some( self.parse_statements_import_as_names() );
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::ImportFromStmt(*start_pos, *end_pos, symbol1, nodes_list, left_node, symbol2, None, right_node, None) )
                            }
                        }
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected 'import' in import statement!", &self.lexer.get_position())
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'from' in import statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_import_as_name(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::AtomName( .. ) => {
                let first_node = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                match &*self.lexer.get_symbol() {
                    Token::PyAs( .. ) => {
                        let symbol = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::AtomName( .. ) => {
                                let last_node = self.lexer.get_symbol();
                                let _ = &self.lexer.advance();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::ImportAsName(*start_pos, *end_pos, first_node, Some((symbol, last_node))) )
                            },
                            _ => {
                                panic!("Syntax Error at {} - Expected Name literal after 'as' in import statement!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::ImportAsName(*start_pos, *end_pos, first_node, None) )
                    }
                }
            }
            _ => {
                panic!("Syntax Error at {} - Expected Name literal in import statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_dotted_as_name(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::AtomName( .. ) => {
                let first_node = self.parse_statements_dotted_name();
                match &*self.lexer.get_symbol() {
                    Token::PyAs( .. ) => {
                        let symbol = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        match &*self.lexer.get_symbol() {
                            Token::AtomName( .. ) => {
                                let last_node = self.lexer.get_symbol();
                                let _ = &self.lexer.advance();
                                let end_pos = &self.lexer.get_position();
                                Box::new( ASTNode::DottedAsNameStmt(*start_pos, *end_pos, first_node, Some((symbol, last_node))) )
                            },
                            _ => {
                                panic!("Syntax Error at {} - Expected Name literal after 'as' in import statement!", &self.lexer.get_position())
                            }
                        }
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::DottedAsNameStmt(*start_pos, *end_pos, first_node, None) )
                    }
                }
            }
            _ => {
                panic!("Syntax Error at {} - Expected Name literal in import statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_import_as_names(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_statements_import_as_name() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();
                    match &*self.lexer.get_symbol() {
                        Token::PySemiColon( .. ) |
                        Token::Newline( .. ) |
                        Token::EOF( .. ) => {
                            false
                        },
                        _ => {
                            nodes_list.push( self.parse_statements_import_as_name() );
                            true
                        }
                    }
                },
                _ => {
                    false
                }
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::ImportAsNamesStmt(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_statements_dotted_as_names(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_statements_dotted_as_name() );
        while
            match &*self.lexer.get_symbol() {
                Token::PyComa( .. ) => {
                    separators_list.push( self.lexer.get_symbol() );
                    let _ = &self.lexer.advance();    
                    nodes_list.push( self.parse_statements_dotted_as_name() );
                    true
                        
                },
                _ => {
                    false
                }
            } {};
        nodes_list.reverse();
        separators_list.reverse();
        let end_pos = &self.lexer.get_position();
        Box::new( ASTNode::DottedAsNamesStmt(*start_pos, *end_pos, nodes_list, separators_list) )
    }

    fn parse_statements_dotted_name(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match &*self.lexer.get_symbol() {
            Token::AtomName( .. ) => {
                nodes_list.push( self.lexer.get_symbol() );
                let _ = &self.lexer.advance();  
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyDot( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::AtomName( .. ) => {
                                    nodes_list.push( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance();  
                                },
                                _ => {
                                    panic!("Syntax Error at {} - Expected Name literal after '.' in import statement!", &self.lexer.get_position())
                                }
                            }
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::DottedNameStmt(*start_pos, *end_pos, nodes_list, separators_list) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected Name literal in import statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_global_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyGlobal( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                match &*self.lexer.get_symbol() {
                    Token::AtomName( .. ) => {
                        nodes_list.push( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance(); 
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected Name literal in global statement!", &self.lexer.get_position())
                    }
                }
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance(); 
                            match &*self.lexer.get_symbol() {
                                Token::AtomName( .. ) => {
                                    nodes_list.push( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance(); 
                                },
                                _ => {
                                    panic!("Syntax Error at {} - Expected Name literal in global statement!", &self.lexer.get_position())
                                }
                            }
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                nodes_list.reverse();
                separators_list.reverse();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::GlobalStmt(*start_pos, *end_pos, symbol, nodes_list, separators_list) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'global' in global statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_nonlocal_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyNonLocal( .. ) => {
                let symbol = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                match &*self.lexer.get_symbol() {
                    Token::AtomName( .. ) => {
                        nodes_list.push( self.lexer.get_symbol() );
                        let _ = &self.lexer.advance(); 
                    },
                    _ => {
                        panic!("Syntax Error at {} - Expected Name literal in nonlocal statement!", &self.lexer.get_position())
                    }
                }
                while
                    match &*self.lexer.get_symbol() {
                        Token::PyComa( .. ) => {
                            separators_list.push( self.lexer.get_symbol() );
                            let _ = &self.lexer.advance();
                            match &*self.lexer.get_symbol() {
                                Token::AtomName( .. ) => {
                                    nodes_list.push( self.lexer.get_symbol() );
                                    let _ = &self.lexer.advance(); 
                                },
                                _ => {
                                    panic!("Syntax Error at {} - Expected Name literal in nonlocal statement!", &self.lexer.get_position())
                                }
                            }
                            true
                        },
                        _ => {
                            false
                        }
                    } {};
                nodes_list.reverse();
                separators_list.reverse();
                let end_pos = &self.lexer.get_position();
                Box::new( ASTNode::NonLocalStmt(*start_pos, *end_pos, symbol, nodes_list, separators_list) )
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'global' in global statement!", &self.lexer.get_position())
            }
        }
    }

    fn parse_statements_assert_stmt(&self) -> Box<ASTNode> {
        let start_pos = &self.lexer.get_position();
        match &*self.lexer.get_symbol() {
            Token::PyAssert( .. ) => {
                let symbol1 = self.lexer.get_symbol();
                let _ = &self.lexer.advance();
                let left_node = self.parse_expression_test();
                match &*self.lexer.get_symbol() {
                    Token::PyComa( .. ) => {
                        let symbol2 = self.lexer.get_symbol();
                        let _ = &self.lexer.advance();
                        let right_node = self.parse_expression_test();
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AssertStmt(*start_pos, *end_pos, symbol1, left_node, Some( (symbol2, right_node) )) )
                    },
                    _ => {
                        let end_pos = &self.lexer.get_position();
                        Box::new( ASTNode::AssertStmt(*start_pos, *end_pos, symbol1, left_node, None) )
                    }
                }
            },
            _ => {
                panic!("Syntax Error at {} - Expected 'assert' in assert statement!", &self.lexer.get_position())
            }
        }
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