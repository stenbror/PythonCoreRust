extern crate core;

mod parser;

use parser::nodes::{ASTNode};
use crate::parser::expressions::Expressions;
use crate::parser::statements::Statements;
use crate::parser::tokenizer::{PythonCoreTokenizer, Tokenizer};
use parser::trivias::Trivia;
use parser::tokens::Token;
use crate::parser::parser::{Parser, PythonCoreParser};

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