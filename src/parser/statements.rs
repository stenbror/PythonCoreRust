
use crate::{ASTNode, Token };
use crate::parser::blocks::Blocks;
use crate::parser::parser::{ET, Parser, PythonCoreParser};
use crate::parser::expressions::Expressions;
use crate::parser::patterns::Patterns;
use crate::parser::tokenizer::Tokenizer;


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
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::AtomName(..) => {
                        let left_node = self.parse_statements_dotted_name()?;
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
                                                        Ok(Box::new( ASTNode::DottedAsNameStmt(start_pos, self.lexer.get_position(), left_node, Some((symbol2, symbol3))) ))
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting name literal in dotted as name statement!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in dotted as name statement!", start_pos))
                                        }
                                    },
                                    _ => {
                                        Ok(Box::new( ASTNode::DottedAsNameStmt(start_pos, self.lexer.get_position(), left_node, None) ))
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in dotted as name statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting name literal in dotted as name statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in dotted as name statement!", start_pos))
        }
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
                    _ => Err(format!("SyntaxError at {}: Expecting 'nonlocal' in nonlocal statement!", start_pos))
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
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyAsync(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s) => {
                                match &*s {
                                    Token::PyDef(..) => {
                                        let right_node = self.parse_blocks_class_def()?;
                                        Ok(Box::new( ASTNode::AsyncStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                                    },
                                    Token::PyWith(..) => {
                                        let right_node = self.parse_statements_with_stmt()?;
                                        Ok(Box::new( ASTNode::AsyncStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                                    },
                                    Token::PyFor(..) => {
                                        let right_node = self.parse_statements_for_stmt()?;
                                        Ok(Box::new( ASTNode::AsyncStmt(start_pos, self.lexer.get_position(), symbol, right_node) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting 'def', 'with' or 'for' in 'async' statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in async statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'async' in async statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in async statement!", start_pos))
        }
    }

    fn parse_statements_if_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyIf(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let left_node = self.parse_expressions_test()?;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_statements_suite()?;
                                        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
                                        while
                                            match self.symbol.clone() {
                                                Ok(s3) => {
                                                    match &*s3 {
                                                        Token::PyElif(..) => {
                                                            nodes_list.push( self.parse_statements_elif_stmt()? );
                                                            true
                                                        },
                                                        _ => false
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in if statement!", start_pos))
                                            } { };
                                        nodes_list.reverse();
                                        match self.symbol.clone() {
                                            Ok(s) => {
                                                match &*s {
                                                    Token::PyElse(..) => {
                                                        let next_node = Some( self.parse_statements_else_stmt()? );
                                                        Ok(Box::new( ASTNode::IfStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, nodes_list, next_node) ))
                                                    },
                                                    _ => {
                                                        Ok(Box::new( ASTNode::IfStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, nodes_list, None) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in if statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in if statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in if statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting symbol in 'if' statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in if statement!", start_pos))
        }
    }

    fn parse_statements_elif_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyElif(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let left_node = self.parse_expressions_test()?;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_statements_suite()?;
                                        Ok(Box::new( ASTNode::ElifStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in elif statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in elif statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'elif' in elif statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in elif statement!", start_pos))
        }
    }

    fn parse_statements_else_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyElse(..) => {
                        let symbol1 = s;
                        let _ = self.advance();

                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_statements_suite()?;
                                        Ok(Box::new( ASTNode::ElseStmt(start_pos, self.lexer.get_position(), symbol1, symbol2, right_node) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in else statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in elif statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'else' in elif statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in else statement!", start_pos))
        }
    }

    fn parse_statements_while_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyWhile(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let left_node = self.parse_expressions_test()?;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_statements_suite()?;
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyElse(..) => {
                                                        let next_node = Some( self.parse_statements_else_stmt()? );
                                                        Ok(Box::new( ASTNode::WhileStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, next_node) ))
                                                    },
                                                    _ => {
                                                        Ok(Box::new( ASTNode::WhileStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, None) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in while statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in while statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in while statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'while' in while statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in while statement!", start_pos))
        }
    }

    fn parse_statements_for_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyFor(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let left_node = self.parse_expressions_exprlist()?;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyIn(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_expressions_testlist()?;
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyColon(..) => {
                                                        let symbol3 = s3;
                                                        let _ = self.advance();
                                                        let mut tc_symbol: Option<Box<Token>> = None;
                                                        match self.symbol.clone() {
                                                            Ok(s4) => {
                                                                match &*s4 {
                                                                    Token::TypeComment(..) => {
                                                                        tc_symbol = Some(s4);
                                                                        let _ = self.advance();
                                                                    },
                                                                    _ => { }
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in for statement!", start_pos))
                                                        }
                                                        let next_node = self.parse_statements_suite()?;
                                                        match self.symbol.clone() {
                                                            Ok(s5) => {
                                                                match &*s5 {
                                                                    Token::PyElse(..) => {
                                                                        let else_node = Some( self.parse_statements_else_stmt()? );
                                                                        Ok(Box::new( ASTNode::ForStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, symbol3, tc_symbol, next_node, else_node) ))
                                                                    },
                                                                    _ => {
                                                                        Ok(Box::new( ASTNode::ForStmt(start_pos, self.lexer.get_position(), symbol1, left_node, symbol2, right_node, symbol3, tc_symbol, next_node, None) ))
                                                                    }
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in for statement!", start_pos))
                                                        }
                                                    },
                                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in for statement!", start_pos))
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in for statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting 'in' in for statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in for statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'for' in for statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in for statement!", start_pos))
        }
    }

    fn parse_statements_try_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyTry(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        self.except_status = ET::ExceptNone;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let left_node = self.parse_statements_suite()?;
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyFinally(..) => {
                                                        let right_node = Some( self.parse_statements_finally_stmt()? );
                                                        Ok(Box::new( ASTNode::TryStmt(start_pos, self.lexer.get_position(), symbol1, symbol2, left_node, None, None, right_node) ))
                                                    },
                                                    _ => {
                                                        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
                                                        let mut else_node : Option<Box<ASTNode>> = None;
                                                        let mut right_node : Option<Box<ASTNode>> = None;
                                                        nodes_list.push( self.parse_statements_except_stmt()? );
                                                        while
                                                            match self.symbol.clone() {
                                                                Ok(s4) => {
                                                                    match &*s4 {
                                                                        Token::PyExcept(..) => {
                                                                            nodes_list.push( self.parse_statements_except_stmt()? );
                                                                            true
                                                                        },
                                                                        _ => false
                                                                    }
                                                                },
                                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in try statement!", start_pos))
                                                            } { };
                                                        match self.symbol.clone() {
                                                            Ok(s5) => {
                                                                match &*s5 {
                                                                    Token::PyElse(..) => {
                                                                        else_node = Some( self.parse_statements_else_stmt()? )
                                                                    },
                                                                    _ => { }
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in try statement!", start_pos))
                                                        }
                                                        match self.symbol.clone() {
                                                            Ok(s6) => {
                                                                match &*s6 {
                                                                    Token::PyFinally(..) => {
                                                                        right_node = Some( self.parse_statements_finally_stmt()? )
                                                                    },
                                                                    _ => { }
                                                                }
                                                            },
                                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in try statement!", start_pos))
                                                        }
                                                        nodes_list.reverse();
                                                        Ok(Box::new( ASTNode::TryStmt(start_pos, self.lexer.get_position(), symbol1, symbol2, left_node, Some( nodes_list ), else_node, right_node) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in try statement!", start_pos))
                                        }
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in try statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in try statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'try' in try statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in try statement!", start_pos))
        }
    }

    fn parse_statements_finally_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyFinally(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        let symbol2 = s2;
                                        let _ = self.advance();
                                        let right_node = self.parse_statements_suite()?;
                                        Ok(Box::new( ASTNode::FinallyStmt(start_pos, self.lexer.get_position(), symbol1, symbol2, right_node) ))
                                    },
                                    _ => Err(format!("SyntaxError at {}: Expecting ':' in finally statement!", start_pos))
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in finally statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'finally' in finally statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in finally statement!", start_pos))
        }
    }

    fn parse_statements_with_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyWith(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
                        let mut separators_list : Box<Vec<Box<Token>>> = Box::new(Vec::new());
                        let mut left_symbol : Option<Box<Token>> = None;
                        let mut right_symbol : Option<Box<Token>> = None;
                        let symbol2;
                        match self.symbol.clone() {
                            Ok(s3) => {
                                match &*s3 {
                                    Token::PyLeftParen(..) => {
                                        left_symbol = Some( s3 );
                                        let _ = self.advance();
                                        nodes_list.push( self.parse_statements_with_item()? );
                                        while
                                            match self.symbol.clone() {
                                                Ok(s5) => {
                                                    match &*s5 {
                                                        Token::PyComa(..) => {
                                                            separators_list.push( s5 );
                                                            let _ = self.advance();
                                                            match self.symbol.clone() {
                                                                Ok(s6) => {
                                                                    match &*s6 {
                                                                        Token::PyRightParen(..) => false,
                                                                        _ => {
                                                                            nodes_list.push( self.parse_statements_with_item()? );
                                                                            true
                                                                        }
                                                                    }
                                                                },
                                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
                                                            }
                                                        },
                                                        _ => false
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
                                            } { };
                                        match self.symbol.clone() {
                                            Ok(s7) => {
                                                match &*s7 {
                                                    Token::PyRightParen(..) => {
                                                        right_symbol = Some( s7 );
                                                        let _ = self.advance();
                                                    },
                                                    _ => return Err(format!("SyntaxError at {}: Expecting ')' in with statement!", start_pos))
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
                                        }
                                    },
                                    _ => {
                                        nodes_list.push( self.parse_statements_with_item()? );
                                        while
                                            match self.symbol.clone() {
                                                Ok(s4) => {
                                                    match &*s4 {
                                                        Token::PyComa(..) => {
                                                            separators_list.push( s4 );
                                                            let _ = self.advance();
                                                            nodes_list.push( self.parse_statements_with_item()? );
                                                            true
                                                        },
                                                        _ => false
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
                                            } { };
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
                        }
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        symbol2 = s2;
                                        let _ = self.advance();
                                    },
                                    _ => return Err(format!("SyntaxError at {}: Expecting ':' in with statement!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
                        }
                        let right_node = self.parse_statements_suite()?;
                        nodes_list.reverse();
                        separators_list.reverse();
                        Ok(Box::new( ASTNode::WithStmt(start_pos, self.lexer.get_position(), symbol1, left_symbol, nodes_list, separators_list, right_symbol, symbol2, right_node) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'with' in finally statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in with statement!", start_pos))
        }
    }

    fn parse_statements_with_item(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left_node = self.parse_expressions_test()?;
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyAs(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_expressions_expr()?;
                        Ok(Box::new( ASTNode::ExceptStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                    },
                    _ => {
                        Ok(Box::new( ASTNode::WithItem(start_pos, self.lexer.get_position(), left_node, None) ))
                    }
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in with item statement!", start_pos))
        }
    }

    fn parse_statements_except_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let left_node = self.parse_statements_except_clause()?;
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyColon(..) => {
                        let symbol = s;
                        let _ = self.advance();
                        let right_node = self.parse_statements_suite()?;
                        Ok(Box::new( ASTNode::ExceptStmt(start_pos, self.lexer.get_position(), left_node, symbol, right_node) ))
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting ':' in except statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in except statement!", start_pos))
        }
    }

    fn parse_statements_except_clause(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        let mut symbol_mul : Option<Box<Token>> = None;
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::PyExcept(..) => {
                        let symbol1 = s;
                        let _ = self.advance();

                        /* Handle 'except*' or 'except' e.g. Python 3.11 */
                        match self.symbol.clone() {
                            Ok(s10) => {
                                match &*s10 {
                                    Token::PyMul(..)=> {
                                        symbol_mul = Some(s10);
                                        let _ = self.advance();
                                        match &self.except_status {
                                            ET::ExceptNone => { self.except_status = ET::ExceptMul },
                                            ET::Except => return Err(format!("SyntaxError at {}: Expecting 'except*' except statement!", start_pos)),
                                            _ => { }
                                        }

                                    },
                                    _ => {
                                        match &self.except_status {
                                            ET::ExceptNone => { self.except_status = ET::Except },
                                            ET::ExceptMul => return Err(format!("SyntaxError at {}: Expecting 'except' except statement!", start_pos)),
                                            _ => { }
                                        }
                                    }
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in except statement!", start_pos))
                        }


                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::PyColon(..) => {
                                        Ok(Box::new( ASTNode::ExceptClauseStmt(start_pos, self.lexer.get_position(), symbol1, symbol_mul, None) ))
                                    },
                                    _ => {
                                        //let symbol2 = s2;  // BUG!
                                        //let _ = self.advance();
                                        let left_node = self.parse_expressions_test()?;
                                        match self.symbol.clone() {
                                            Ok(s3) => {
                                                match &*s3 {
                                                    Token::PyAs(..) => {
                                                        let symbol2 = s3;
                                                        let _ = self.advance();
                                                        match self.symbol.clone() {
                                                            Ok(s4) => {
                                                                match &*s4 {
                                                                    Token::AtomName(..) => {
                                                                        let symbol3 = s4;
                                                                        let _ = self.advance();
                                                                        Ok(Box::new( ASTNode::ExceptClauseStmt(start_pos, self.lexer.get_position(), symbol1, symbol_mul,Some( ( left_node, Some( ( symbol2, symbol3 ) ) ) )) ))
                                                                    },
                                                                    _ => Err(format!("SyntaxError at {}: Expecting name literal after 'as' in except statement!", start_pos))
                                                                }
                                                            },
                                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in except clause statement!", start_pos))
                                                        }
                                                    },
                                                    _ => {
                                                        Ok(Box::new( ASTNode::ExceptClauseStmt(start_pos, self.lexer.get_position(),symbol1, symbol_mul, Some( ( left_node, None ) )) ))
                                                    }
                                                }
                                            },
                                            _ => Err(format!("SyntaxError at {}: Expecting symbol in except statement!", start_pos))
                                        }
                                    }
                                }
                            },
                            _ => Err(format!("SyntaxError at {}: Expecting symbol in except statement!", start_pos))
                        }
                    },
                    _ => Err(format!("SyntaxError at {}: Expecting 'except' in except statement!", start_pos))
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in except clause statement!", start_pos))
        }
    }

    fn parse_statements_suite(&mut self) -> Result<Box<ASTNode>, String> {
        let start_pos = self.lexer.get_position();
        match self.symbol.clone() {
            Ok(s) => {
                match &*s {
                    Token::Newline(..) => {
                        let symbol1 = s;
                        let _ = self.advance();
                        let mut nodes_list : Box<Vec<Box<ASTNode>>> = Box::new(Vec::new());
                        let symbol2;
                        let symbol3;
                        match self.symbol.clone() {
                            Ok(s2) => {
                                match &*s2 {
                                    Token::Indent(..) => {
                                        symbol2 = s2;
                                        let _ = self.advance();
                                        nodes_list.push(self.parse_statements_stmt()?);
                                        while
                                        match self.symbol.clone() {
                                                Ok(s3) => {
                                                    match &*s3 {
                                                        Token::Dedent(..) => false,
                                                        _ => {
                                                            nodes_list.push(self.parse_statements_stmt()?);
                                                            true
                                                        }
                                                    }
                                                },
                                                _ => return Err(format!("SyntaxError at {}: Expecting symbol in suite statement!", start_pos))
                                            } { };
                                        match self.symbol.clone() {
                                            Ok(s4) => {
                                                match &*s4 {
                                                    Token::Dedent(..) => {
                                                        symbol3 = s4;
                                                        let _ = self.advance();
                                                    },
                                                    _ => return Err(format!("SyntaxError at {}: Expecting dedent in suite statement!", start_pos))
                                                }
                                            },
                                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in suite statement!", start_pos))
                                        }
                                    },
                                    _ => return Err(format!("SyntaxError at {}: Expecting indentation in suite statement!", start_pos))
                                }
                            },
                            _ => return Err(format!("SyntaxError at {}: Expecting symbol in suite statement!", start_pos))
                        }
                        nodes_list.reverse();
                        Ok(Box::new( ASTNode::SuiteStmt(start_pos, self.lexer.get_position(), symbol1, symbol2, nodes_list, symbol3) ))
                    },
                    _ => {
                        self.parse_statements_simple_stmt()
                    }
                }
            },
            _ => Err(format!("SyntaxError at {}: Expecting symbol in suite statement!", start_pos))
        }
    }
}


// UnitTests for statements rules /////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use crate::{ASTNode, Parser, PythonCoreParser, PythonCoreTokenizer, Statements, Token, Tokenizer};

    #[test]
    fn statements_expression_less_operator() {
        let lexer = Box::new(PythonCoreTokenizer::new("a < b\r\n".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_statements_stmt();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::SimpleStmtList(0, 8, nodes, separators, nwl) => {
                        assert_eq!(nodes.len(), 1);
                        let node = (*nodes[0]).clone();
                        match node {
                            ASTNode::LessComparison( 0 , 5 , left, symbol, right) =>  {
                                match &*left {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*symbol {
                                    Token::PyLess(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*right {
                                    ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                assert!(true)
                            },
                            _ => assert!(false)
                        }
                        assert_eq!(separators.len(), 0);
                        match &**nwl {
                            Token::Newline(5, 8, None, '\r', '\n') =>  assert!(true),
                            _ => assert!(false)
                        }
                   },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn statements_expression_greater_operator() {
        let lexer = Box::new(PythonCoreTokenizer::new("a > b\r\n".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_statements_stmt();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::SimpleStmtList(0, 8, nodes, separators, nwl) => {
                        assert_eq!(nodes.len(), 1);
                        let node = (*nodes[0]).clone();
                        match node {
                            ASTNode::GreaterComparison( 0 , 5 , left, symbol, right) =>  {
                                match &*left {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*symbol {
                                    Token::PyGreater(2, 3, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*right {
                                    ASTNode::AtomName( 4, 5 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                assert!(true)
                            },
                            _ => assert!(false)
                        }
                        assert_eq!(separators.len(), 0);
                        match &**nwl {
                            Token::Newline(5, 8, None, '\r', '\n') =>  assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn statements_expression_less_equal_operator() {
        let lexer = Box::new(PythonCoreTokenizer::new("a <= b\r\n".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_statements_stmt();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::SimpleStmtList(0, 9, nodes, separators, nwl) => {
                        assert_eq!(nodes.len(), 1);
                        let node = (*nodes[0]).clone();
                        match node {
                            ASTNode::LessEqualComparison( 0 , 6 , left, symbol, right) =>  {
                                match &*left {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*symbol {
                                    Token::PyLessEqual(2, 4, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*right {
                                    ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                assert!(true)
                            },
                            _ => assert!(false)
                        }
                        assert_eq!(separators.len(), 0);
                        match &**nwl {
                            Token::Newline(6, 9, None, '\r', '\n') =>  assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn statements_expression_greater_equal_operator() {
        let lexer = Box::new(PythonCoreTokenizer::new("a >= b\r\n".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_statements_stmt();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::SimpleStmtList(0, 9, nodes, separators, nwl) => {
                        assert_eq!(nodes.len(), 1);
                        let node = (*nodes[0]).clone();
                        match node {
                            ASTNode::GreaterEqualComparison( 0 , 6 , left, symbol, right) =>  {
                                match &*left {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*symbol {
                                    Token::PyGreaterEqual(2, 4, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*right {
                                    ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                assert!(true)
                            },
                            _ => assert!(false)
                        }
                        assert_eq!(separators.len(), 0);
                        match &**nwl {
                            Token::Newline(6, 9, None, '\r', '\n') =>  assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

    #[test]
    fn statements_expression_equal_operator() {
        let lexer = Box::new(PythonCoreTokenizer::new("a == b\r\n".to_string()));
        let mut parser = PythonCoreParser::new(lexer);
        parser.advance();
        let res = parser.parse_statements_stmt();
        match &res {
            Ok(s) => {
                match &**s {
                    ASTNode::SimpleStmtList(0, 9, nodes, separators, nwl) => {
                        assert_eq!(nodes.len(), 1);
                        let node = (*nodes[0]).clone();
                        match node {
                            ASTNode::EqualComparison( 0 , 6 , left, symbol, right) =>  {
                                match &*left {
                                    ASTNode::AtomName( 0, 2 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*symbol {
                                    Token::PyEqual(2, 4, _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                match &*right {
                                    ASTNode::AtomName( 5, 6 , _ ) => assert!(true),
                                    _ => assert!(false)
                                }
                                assert!(true)
                            },
                            _ => assert!(false)
                        }
                        assert_eq!(separators.len(), 0);
                        match &**nwl {
                            Token::Newline(6, 9, None, '\r', '\n') =>  assert!(true),
                            _ => assert!(false)
                        }
                    },
                    _ => assert!(false)
                }
            }
            Err(..) => assert!(false)
        }
    }

}

