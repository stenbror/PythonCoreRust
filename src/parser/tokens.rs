
use crate::parser::trivias::{ Trivia };

pub enum Token {
    PyFalse(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNone(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyTrue(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAnd(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAs(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAssert(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAsync(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyAwait(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyBreak(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyClass(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyContinue(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDef(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyDel(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyElif(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyElse(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyExcept(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFinally(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFor(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyFrom(u32, u32, Option<Box<[Box<Trivia>]>>),
    PÅyGlobal(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyIf(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyImport(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyIn(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyIs(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyLambda(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNonLocal(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyNot(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyOr(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyPass(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyRaise(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyReturn(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyTry(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyWhile(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyWith(u32, u32, Option<Box<[Box<Trivia>]>>),
    PyYield(u32, u32, Option<Box<[Box<Trivia>]>>),

    PyColon(u32, u32, Option<Box<[Box<Trivia>]>>, Box<[Trivia]>),

    Name(u32, u32, Option<Box<[Box<Trivia>]>>, Box<String>)
} 