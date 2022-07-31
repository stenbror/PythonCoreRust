extern crate core;

mod parser;

use std::ops::Deref;
use parser::nodes::{ ASTNode };
use parser::tokens::{ Token };
use crate::parser::parser::{Parser, PythonCoreParser};
use crate::parser::tokenizer::PythonCoreTokenizer;

fn main() {
    println!("Test the Rust!");
    let _res = ASTNode::AtomName(0, 5, Box::new( Token::AtomName( 0, 5, None, Box::new("Test".to_string() ))));
    let tokenizer = PythonCoreTokenizer::new( "test".to_string());
    let symbol = tokenizer.get_symbol();
    match &*symbol {
        Token::AtomName( s, e, _ , txt ) => {
            print!("Token: AtomName, at start: {} and end: {} with text: {}", s, e, txt)
        },
        _ => {}
    }
    let _parser = PythonCoreParser::new( Box::new( tokenizer ) );
}