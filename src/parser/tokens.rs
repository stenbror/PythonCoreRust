
use crate::parser::trivias::{ Trivia };

pub enum Token {
    PyColon(u32, u32, Box<[Trivia]>),

    Name(u32, u32, Box<String>)
} 