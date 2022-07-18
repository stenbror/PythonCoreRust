
use crate::parser::nodes::{ ASTNode };
use crate::parser::tokens::{ Token };
use crate::parser::parser::{ PythonCoreParser };


trait Statements {
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
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_simple_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_small_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_expr_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_ann_assign(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }
    
    fn parse_statements_del_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_pass_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_flow_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_break_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_continue_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_return_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
    }

    fn parse_statements_yield_stmt(&self) -> Box<ASTNode> {
        Box::new( ASTNode::Empty )
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