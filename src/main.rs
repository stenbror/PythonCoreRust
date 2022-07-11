
mod parser;

use parser::nodes::{ ASTNode };
use parser::tokens::{ Token };

fn main() {
    println!("Test the Rust!");
    let _res = ASTNode::AtomName(0, 5, Box::new( Token::Name( 0, 5, Box::new("Test".to_string() ))));
}