
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
    fn parse_statements_ann_assign(&mut self, start_pos: &u32, left_node: Box<ASTNode>) -> Result<Box<ASTNode>, String>;
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
        todo!()
    }

    fn parse_statements_ann_assign(&mut self, start_pos: &u32, left_node: Box<ASTNode>) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_del_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_pass_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_flow_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_break_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_continue_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
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