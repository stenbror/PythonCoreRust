
#[derive(Clone)]
pub enum Trivia {
    WhiteSpace(u32, u32, char),
    Newline(u32, u32, char, char),
    LineContinuation(u32, u32, char, char, char),
    Comment(u32, u32, String),
}