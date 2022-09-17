extern crate core;

mod parser;
mod result_parser;

use crate::parser::nodes::{ASTNode};
use crate::result_parser::expressions::Expressions;
use crate::result_parser::statements::Statements;
use crate::result_parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
use crate::parser::trivias::Trivia;
use crate::parser::tokens::Token;
use crate::result_parser::parser::{Parser, PythonCoreParser};

fn main() {
    println!("PythonCore written in Rust!");

    let mut lexer = Box::new( PythonCoreTokenizer::new("__init__".to_string()) );
    let mut parser = PythonCoreParser::new(lexer);
    parser.advance();
    let res = parser.parse_expressions_atom_expr();

    match &res {
        Ok(s) => {
            match &**s {
                ASTNode::AtomName( 0, 8, tok) => {
                    match &**tok {
                        Token::AtomName(0, 8, None, txt) => {
                            match &*txt.as_str() {
                                "__init__" => { println!("Oh yeah!") },
                                _ => { println!("Failed!"); }
                            }
                        },
                        _ => { println!("Failed!"); }
                    }
                },
                _ => { println!("Failed!"); }
            }
        }
        Err( .. ) => { println!("Failed!"); }
    }
}