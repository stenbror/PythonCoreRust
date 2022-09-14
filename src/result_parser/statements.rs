
use crate::{ASTNode, Token };
use crate::result_parser::parser::{ Parser, PythonCoreParser };
use crate::result_parser::expressions::Expressions;
use crate::result_parser::patterns::Patterns;
use crate::result_parser::tokenizer::Tokenizer;


pub trait Statements {
    fn parse_statements_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_simple_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_small_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_expr_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_ann_assign(&mut self, start_pos: u32, left_node: Box<ASTNode>) -> Result<Box<ASTNode>, String>;
    fn parse_statements_del_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_pass_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_flow_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_break_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_continue_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_return_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_yield_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_raise_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_import_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_import_name(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_import_from(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_import_as_name(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_dotted_as_name(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_import_as_names(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_dotted_as_names(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_dotted_name(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_global_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_nonlocal_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_assert_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_compound_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_async_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_if_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_elif_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_else_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_while_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_for_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_try_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_finally_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_with_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_with_item(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_except_stmt(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_except_clause(&mut self) -> Result<Box<ASTNode>, String>;
    fn parse_statements_suite(&mut self) -> Result<Box<ASTNode>, String>;
}


impl Statements for PythonCoreParser {
    fn parse_statements_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyIf(..) |
                    Token::PyFor(..) |
                    Token::PyWhile(..) |
                    Token::PyWith(..) |
                    Token::PyTry(..) |
                    Token::PyAsync(..) |
                    Token::PyMatrice(..) |
                    Token::PyDef(..) |
                    Token::PyClass(..) => {
                        self.parse_statements_compound_stmt()
                    },
                    Token::AtomName(_, _, _, txt) => {
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
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in statement", start_pos))
        }
    }

    fn parse_statements_simple_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list: Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list: Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push(self.parse_statements_small_stmt()?);
        while
            match self.symbol.clone() {
                Ok(s) => {
                    match &*s {
                        Token::PySemiColon(..) => {
                            separators_list.push(s);
                            let _ = self.advance();
                            match self.symbol.clone() {
                                Ok(s2) => {
                                    match &*s2 {
                                        Token::Newline(..) |
                                        Token::EOF(..) => {
                                            false
                                        },
                                        _ => {
                                            nodes_list.push(self.parse_statements_small_stmt()? );
                                            true
                                        }
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in statement list!", start_pos))
                            }
                        },
                        _ => {
                            false
                        }
                    }
                },
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in statement list!", start_pos))
            } {};
        match self.symbol.clone() {
            Ok(s3) => {
                match &*s3 {
                    Token::Newline( .. ) => {
                        let _ = self.advance();
                        nodes_list.reverse();
                        separators_list.reverse();
                        Ok( Box::new( ASTNode::SimpleStmtList(start_pos, self.lexer.get_position(), nodes_list, separators_list, s3) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting Newline at end of statement list!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in statement list!", start_pos))
        }
    }

    fn parse_statements_small_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyDel(..) => {
                        self.parse_statements_del_stmt()
                    },
                    Token::PyPass(..) => {
                        self.parse_statements_pass_stmt()
                    },
                    Token::PyBreak(..) |
                    Token::PyContinue(..) |
                    Token::PyReturn(..) |
                    Token::PyRaise(..) |
                    Token::PyYield(..) => {
                        self.parse_statements_flow_stmt()
                    },
                    Token::PyImport(..) |
                    Token::PyFrom(..) => {
                        self.parse_statements_import_stmt()
                    },
                    Token::PyGlobal(..) => {
                        self.parse_statements_global_stmt()
                    },
                    Token::PyNonLocal(..) => {
                        self.parse_statements_nonlocal_stmt()
                    },
                    Token::PyAssert(..) => {
                        self.parse_statements_assert_stmt()
                    },
                    _ => {
                        self.parse_statements_expr_stmt()
                    }
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in simple statement!", start_pos))
        }
    }

    fn parse_statements_expr_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left_node = self.parse_expressions_testlist_star_expr()?;
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyColon(..) => {
                        self.parse_statements_ann_assign(start_pos, left_node)
                    },
                    Token::PyAssign(..) => {
                        let mut nodes_list : Box<Vec<Box< ( Box<Token>, Box<ASTNode> ) >>> = Box::new(Vec::new());
                        while
                            match self.symbol.clone() {
                                Ok(s2) => {
                                    match &*s2 {
                                        Token::PyAssign( .. ) => {
                                            let ass_symbol = s2;
                                            let _ = self.advance();
                                            match self.symbol.clone() {
                                                Ok(s3) => {
                                                    match &*s3 {
                                                        Token::PyYield( .. ) => {
                                                            let right_node = self.parse_expressions_yield_expr()?;
                                                            nodes_list.push( Box::new( ( ass_symbol, right_node ) ) )
                                                        },
                                                        _ => {
                                                            let right_node = self.parse_expressions_testlist()?;
                                                            nodes_list.push( Box::new( ( ass_symbol, right_node ) ) )
                                                        }
                                                    }
                                                    true
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                                            }
                                        },
                                        _ => false
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                            } {};
                        nodes_list.reverse();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::TypeComment(..) => {
                                        let tc_symbol = s2;
                                        let _ = self.advance();
                                        Ok(Box::new( ASTNode::AssignmentStmt(start_pos, self.lexer.get_position(), left_node, nodes_list, Some(tc_symbol)) ))
                                    },
                                    _ => {
                                        Ok(Box::new( ASTNode::AssignmentStmt(start_pos, self.lexer.get_position(), left_node, nodes_list, None) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyPlusAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::PlusAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::PlusAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyMinusAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::MinusAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::MinusAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyMulAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::MulAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::MulAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyPowerAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::PowerAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::PowerAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyDivAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::DivAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::DivAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyFloorDivAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::FloorDivAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::FloorDivAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyModuloAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::ModuloAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::ModuloAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyMatriceAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::MatriceAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::MatriceAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyBitAndAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::BitAndAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::BitAndAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyBitOrAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::BitOrAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::BitOrAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyBitXorAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::BitXorAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::BitXorAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyShiftLeftAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::ShiftLeftAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::ShiftLeftAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    Token::PyShiftRightAssign( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyYield( .. ) => {
                                        let right_node = self.parse_expressions_yield_expr()?;
                                        Ok(Box::new( ASTNode::ShiftRightAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    },
                                    _ => {
                                        let right_node = self.parse_expressions_testlist()?;
                                        Ok(Box::new( ASTNode::ShiftRightAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                        }
                    },
                    _ => {
                        Ok(left_node)
                    }
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
        }
    }

    fn parse_statements_ann_assign(&mut self, start_pos: u32, left_node: Box<ASTNode>) -> Result<Box<ASTNode>, String> {
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyColon( .. ) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_expressions_test()?;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyAssign( .. ) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyYield( .. ) => {
                                                        let next_node = self.parse_expressions_yield_expr()?;
                                                        Ok(Box::new( ASTNode::AnnAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node, Some( (symbol2, next_node) )) ))
                                                    },
                                                    _ => {
                                                        let next_node = self.parse_expressions_testlist()?;
                                                        Ok(Box::new( ASTNode::AnnAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node, Some( (symbol2, next_node) )) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in expression statement!", start_pos))
                                        }
                                    },
                                    _ => {
                                        Ok(Box::new( ASTNode::AnnAssignStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node, None) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in annotation assignment statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting ':' in annotation assignment statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in annotation assignment statement!", start_pos))
        }
    }

    fn parse_statements_del_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyDel(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_expressions_exprlist()?;
                        Ok(Box::new( ASTNode::DelStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'del' in del statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'del' statement!", start_pos))
        }
    }

    fn parse_statements_pass_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyPass(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        Ok(Box::new( ASTNode::PassStmt(start_pos, self.lexer.get_position(), symbol) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'pass' in pass statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'pass' statement!", start_pos))
        }
    }

    fn parse_statements_flow_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
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
                    _ => Err(format!("SyntaxError at {}: Expecting flow statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in flow statement!", start_pos))
        }
    }

    fn parse_statements_break_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyBreak(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        Ok(Box::new( ASTNode::BreakStmt(start_pos, self.lexer.get_position(), symbol) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'break' in break statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'break' statement!", start_pos))
        }
    }

    fn parse_statements_continue_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyContinue(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        Ok(Box::new( ASTNode::ContinueStmt(start_pos, self.lexer.get_position(), symbol) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'continue' in pass statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'continue' statement!", start_pos))
        }
    }

    fn parse_statements_return_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_yield_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_raise_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_import_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_import_name(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_import_from(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_import_as_name(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_dotted_as_name(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_import_as_names(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_dotted_as_names(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_dotted_name(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_global_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_nonlocal_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_assert_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_compound_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_async_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_if_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_elif_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_else_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_while_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_for_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_try_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_finally_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_with_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_with_item(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_except_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_except_clause(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_suite(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }
}


// UnitTests for statements rules /////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Token};
    use crate::result_parser::expressions::Expressions;
    use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
    use crate::parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn statements_empty_template() {
        assert!(true)
    }

}