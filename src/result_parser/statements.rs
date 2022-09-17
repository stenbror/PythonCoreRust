
use crate::{ASTNode, Token };
use crate::result_parser::blocks::Blocks;
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
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyReturn(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PySemiColon( .. ) |
                                    Token::Newline( .. ) |
                                    Token::EOF( .. ) => {
                                        Ok(Box::new( ASTNode::ReturnStmt(start_pos, self.lexer.get_position(), symbol, None) ))
                                    },
                                    _ => {
                                        let right_node = Some( self.parse_expressions_testlist_star_expr()? );
                                        Ok(Box::new( ASTNode::ReturnStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'return' statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'return' in 'return' statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'return' statement!", start_pos))
        }
    }

    fn parse_statements_yield_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        self.parse_expressions_yield_expr()
    }

    fn parse_statements_raise_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyRaise(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PySemiColon( .. ) |
                                    Token::Newline( .. ) |
                                    Token::EOF( .. ) => {
                                        Ok(Box::new( ASTNode::RaiseStmt(start_pos, self.lexer.get_position(), symbol1, None) ))
                                    },
                                    _ => {
                                        let left_node = self.parse_expressions_test()?;
                                        match self.symbol.clone() {
                                            Ok(s2) => {
                                                match &*s2 {
                                                    Token::PyFrom(..) => {
                                                        let symbol2 = s2;
                                                        let _ = self.advance();
                                                        let right_node = self.parse_expressions_test()?;
                                                        Ok(Box::new( ASTNode::RaiseStmt(start_pos, self.lexer.get_position(), symbol1, Some( ( left_node, Some( ( symbol2, right_node) )) )) ))
                                                    },
                                                    _ => {
                                                        Ok(Box::new( ASTNode::RaiseStmt(start_pos, self.lexer.get_position(), symbol1, Some( ( left_node, None) )) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting Symbol in 'raise' statement!", start_pos))
                                        }
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting Symbol in 'raise' statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'raise' in 'raise' statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting Symbol in 'raise' statement!", start_pos))
        }
    }

    fn parse_statements_import_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyImport(..) => {
                        self.parse_statements_import_stmt()
                    },
                    Token::PyFrom(..) => {
                        self.parse_statements_import_from()
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'import' or 'from' in statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'import' or 'from' statement!", start_pos))
        }
    }

    fn parse_statements_import_name(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyImport(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_statements_dotted_as_names()?;
                        Ok( Box::new( ASTNode::ImportNameStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'import' in import statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'import' statement!", start_pos))
        }
    }

    fn parse_statements_import_from(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyFrom(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                        while
                            match self.symbol.clone() {
                                Ok(s2) => {
                                    match &*s2 {
                                        Token::PyDot(..) |
                                        Token::PyElipsis(..) => {
                                            nodes_list.push( s2 );
                                            let _ = self.advance();
                                            true
                                        },
                                        _ => {
                                            false
                                        }
                                    }
                                },
                                _ => false
                            } { };
                        nodes_list.reverse();
                        let mut left_node : Option<Box<ASTNode>> = None;
                        match self.symbol.clone() {
                            Ok(s3) => {
                                match &*s3 {
                                    Token::PyImport(..) => {
                                        match nodes_list.len() {
                                            0 => return Err(format!("SyntaxError at {}: Expecting '.' in from part of import statement!", start_pos)),
                                            _ => { }
                                        }
                                    },
                                    _ => {
                                        left_node =  Some( self.parse_statements_dotted_name()?);
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in 'from' statement!", start_pos))
                        }
                        match self.symbol.clone() {
                            Ok(s4) => {
                                match &*s4 {
                                    Token::PyImport(..) => {
                                        let symbol2 = s4;
                                        let _ = self.advance();
                                        match self.symbol.clone() {
                                            Ok(s5) => {
                                                match &*s5 {
                                                    Token::PyMul(..) => {
                                                        let symbol3 = s5;
                                                        let _ = self.advance();
                                                        Ok(Box::new( ASTNode::ImportFromStmt(start_pos, self.lexer.get_position(), symbol1, nodes_list, left_node, symbol2, Some(symbol3), None, None) ))
                                                    },
                                                    Token::PyLeftParen(..) => {
                                                        let symbol3 = s5;
                                                        let _ = self.advance();
                                                        let right_node = Some( self.parse_statements_import_as_names()? );
                                                        match self.symbol.clone() {
                                                            Ok(s6) => {
                                                                match &*s6 {
                                                                    Token::PyRightParen(..) => {
                                                                        let symbol4 = s6;
                                                                        let _ = self.advance();
                                                                        Ok(Box::new( ASTNode::ImportFromStmt(start_pos, self.lexer.get_position(), symbol1, nodes_list, left_node, symbol2, Some(symbol3), right_node, Some(symbol4)) ))
                                                                    },
                                                                    _ => Err(format!("SyntaxError at {}: Expecting ')' in from import statement!", start_pos))
                                                                }
                                                            },
                                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'from' statement!", start_pos))
                                                        }
                                                    },
                                                    _ => {
                                                        let right_node = Some( self.parse_statements_import_as_names()? );
                                                        Ok(Box::new( ASTNode::ImportFromStmt(start_pos, self.lexer.get_position(), symbol1, nodes_list, left_node, symbol2, None, right_node, None) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'from' statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting symbol in 'from' statement!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting 'import' in from import statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'from' in import statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'from' statement!", start_pos))
        }
    }

    fn parse_statements_import_as_name(&mut self) -> Result<Box<ASTNode>, String> {
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
                                    Token::PyAs(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::AtomName(..) => {
                                                        let symbol3 = s3;
                                                        let _ = self.advance();
                                                        Ok(Box::new( ASTNode::ImportAsName(start_pos, self.lexer.get_position(), symbol1, Some((symbol2, symbol3))) ))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting name literal in import as name statement!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in import as name statement!", start_pos))
                                        }
                                    },
                                    _ => {
                                        Ok(Box::new( ASTNode::ImportAsName(start_pos, self.lexer.get_position(), symbol1, None) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in import as name statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting name literal in import as name statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in import as name statement!", start_pos))
        }
    }

    fn parse_statements_dotted_as_name(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_import_as_names(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_statements_import_as_name()? );
        while
            match self.symbol.clone() {
                Ok(s) => {
                    match &*s {
                        Token::PyComa(..) => {
                            separators_list.push( s );
                            let _ = self.advance();
                            nodes_list.push( self.parse_statements_import_as_name()? );
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in import as names statement!", start_pos))
            } { };
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new( ASTNode::ImportAsNamesStmt(start_pos, self.lexer.get_position(), nodes_list, separators_list) ))
    }

    fn parse_statements_dotted_as_names(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        nodes_list.push( self.parse_statements_dotted_as_name()? );
        while
            match self.symbol.clone() {
                Ok(s) => {
                    match &*s {
                        Token::PyComa(..) => {
                            separators_list.push( s );
                            let _ = self.advance();
                            nodes_list.push( self.parse_statements_dotted_as_name()? );
                            true
                        },
                        _ => false
                    }
                },
                _ => return Err(format!("SyntaxError at {}: Expecting symbol in dotted as name statement!", start_pos))
            } { };
        nodes_list.reverse();
        separators_list.reverse();
        Ok(Box::new( ASTNode::DottedAsNamesStmt(start_pos, self.lexer.get_position(), nodes_list, separators_list) ))
    }

    fn parse_statements_dotted_name(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::AtomName(..) => {
                        nodes_list.push( s );
                        let _ = self.advance();
                        while
                            match self.symbol.clone() {
                                Ok(s2) => {
                                    match &*s2 {
                                        Token::PyDot(..) => {
                                            separators_list.push( s2 );
                                            let _ = self.advance();
                                            match self.symbol.clone() {
                                                Ok(s3) => {
                                                    match &*s3 {
                                                        Token::AtomName(..) => {
                                                            nodes_list.push(s3);
                                                            let _ = self.advance();
                                                        },
                                                        _ => return Err(format!("SyntaxError at {}: Expecting literal name in import statement!", start_pos))
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in dotted name statement!", start_pos))
                                            }
                                            true
                                        },
                                        _ => false
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in dotted name statement!", start_pos))
                            } { };
                        nodes_list.reverse();
                        separators_list.reverse();
                        Ok(Box::new( ASTNode::DottedNameStmt(start_pos, self.lexer.get_position(), nodes_list, separators_list) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting literal name in import statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in dotted name statement!", start_pos))
        }
    }

    fn parse_statements_global_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyGlobal(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::AtomName(..) => {
                                        nodes_list.push( s2 );
                                        let _ = self.advance();
                                    },
                                    _ => return Err(format!("SyntaxError at {}: Expecting name literal in global statement!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in 'global' statement!", start_pos))
                        }
                        while
                            match self.symbol.clone() {
                                Ok(s3) => {
                                    match &*s3 {
                                        Token::PyComa(..) => {
                                            separators_list.push( s3 );
                                            let _ = self.advance();
                                            match self.symbol.clone() {
                                                Ok(s4) => {
                                                    match &*s4 {
                                                        Token::AtomName(..) => {
                                                            nodes_list.push( s4 );
                                                            let _ = self.advance();
                                                        },
                                                        _ => return Err(format!("SyntaxError at {}: Expecting name literal in global statement!", start_pos))
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting 'global' in global statement!", start_pos))
                                            }
                                            true
                                        },
                                        _ => false
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in 'global' statement!", start_pos))
                            } { };
                        nodes_list.reverse();
                        separators_list.reverse();
                        Ok(Box::new( ASTNode::GlobalStmt(start_pos, self.lexer.get_position(), symbol, nodes_list, separators_list) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'global' in global statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'global' statement!", start_pos))
        }
    }

    fn parse_statements_nonlocal_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyNonLocal(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let mut nodes_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::AtomName(..) => {
                                        nodes_list.push( s2 );
                                        let _ = self.advance();
                                    },
                                    _ => return Err(format!("SyntaxError at {}: Expecting name literal in nonlocal statement!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in 'nonlocal' statement!", start_pos))
                        }
                        while
                            match self.symbol.clone() {
                                Ok(s3) => {
                                    match &*s3 {
                                        Token::PyComa(..) => {
                                            separators_list.push( s3 );
                                            let _ = self.advance();
                                            match self.symbol.clone() {
                                                Ok(s4) => {
                                                    match &*s4 {
                                                        Token::AtomName(..) => {
                                                            nodes_list.push( s4 );
                                                            let _ = self.advance();
                                                        },
                                                        _ => return Err(format!("SyntaxError at {}: Expecting name literal in nonlocal statement!", start_pos))
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting 'nonlocal' in global statement!", start_pos))
                                            }
                                            true
                                        },
                                        _ => false
                                    }
                                },
                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in 'nonlocal' statement!", start_pos))
                            } { };
                        nodes_list.reverse();
                        separators_list.reverse();
                        Ok(Box::new( ASTNode::NonLocalStmt(start_pos, self.lexer.get_position(), symbol, nodes_list, separators_list) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'nonlocaL' in nonlocal statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'nonlocal' statement!", start_pos))
        }
    }

    fn parse_statements_assert_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyAssert(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let left_node = self.parse_expressions_test()?;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyFrom(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_test()?;
                                        Ok(Box::new( ASTNode::AssertStmt(start_pos, self.lexer.get_position(), symbol1, left_node, Some( (symbol2, right_node) )) ))
                                    },
                                    _ => {
                                        Ok(Box::new( ASTNode::AssertStmt(start_pos, self.lexer.get_position(), symbol1, left_node, None) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'assert' statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'assert' in assert statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in 'assert' statement!", start_pos))
        }
    }

    fn parse_statements_compound_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyIf( .. ) => {
                        self.parse_statements_if_stmt()
                    },
                    Token::PyWhile( .. ) => {
                        self.parse_statements_while_stmt()
                    },
                    Token::PyFor( .. ) => {
                        self.parse_statements_for_stmt()
                    },
                    Token::PyTry( .. ) => {
                        self.parse_statements_try_stmt()
                    },
                    Token::PyWith( .. ) => {
                        self.parse_statements_with_stmt()
                    },
                    Token::PyDef( .. ) => {
                        self.parse_blocks_func_def()
                    },
                    Token::PyClass( .. ) => {
                        self.parse_blocks_class_def()
                    },
                    Token::PyMatrice( .. ) => {
                        self.parse_blocks_decorated()
                    },
                    Token::PyAsync( .. ) => {
                        self.parse_statements_async_stmt()
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting compound statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in compound statement!", start_pos))
        }
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
    use crate::result_parser::trivias::Trivia;
    use crate::result_parser::parser::{Parser, PythonCoreParser};


    #[test]
    fn statements_empty_template() {
        assert!(true)
    }

}