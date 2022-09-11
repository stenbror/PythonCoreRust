
use crate::{ASTNode, Token };
use crate::result_parser::parser::{ Parser, PythonCoreParser };
use crate::result_parser::expressions::Expressions;
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
        todo!()
    }

    fn parse_statements_simple_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
    }

    fn parse_statements_small_stmt(&mut self) -> Result<Box<ASTNode>, String> {
        todo!()
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