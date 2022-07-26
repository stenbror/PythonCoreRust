
struct SourceBuffer {
    source_buffer: Box<Vec<char>>,
    index: u32
}

impl SourceBuffer {

    /// Construct new source buffer from given string containing sourcefile from file, evalinput or console input
    fn new(buffer: String) -> SourceBuffer {
        SourceBuffer {
            source_buffer: Box::new(buffer.chars().collect()),
            index: 0
        }
    }

    /// Returns current character at position into source buffer, does not advance by itself, so we can read it many times if needed
    pub fn get_char(&self) -> &char {
        let index_local = *&self.index as usize;
        let max = *&self.source_buffer.len() as usize;
        if (index_local < max) { &*&(self.source_buffer[index_local]) } else { &*&(self.source_buffer[max - 2]) }
    }

    /// Returns a peek into current position and next two position in source buffer, without any advance. At end it will just return eof as character.
    pub fn peek_three_chars(&self) -> ( &char, &char, &char ) {
        let index_local = *&self.index as usize;
        let max = *&self.source_buffer.len() as usize;
        let a = if (index_local < max) { &*&(self.source_buffer[index_local]) } else { &*&(self.source_buffer[max - 2]) };
        let b = if ((index_local + 1) < max) { &*&(self.source_buffer[index_local + 1]) } else { &*&(self.source_buffer[max - 2]) };
        let c = if ((index_local + 2) < max) { &*&(self.source_buffer[index_local + 2]) } else { &*&(self.source_buffer[max - 2]) };
        ( &a, &b, &c )
    }

    /// This moves the position into source buffer one char ahead.
    pub fn advance(&mut self) -> () {
        self.index += 1;
    }

    /// Get current position inbto source buffer
    pub fn get_position(&self) -> &u32 {
        &(self.index)
    }

    /// Returns the maximum characters in source buffer. length of vector.
    pub fn count(&self) -> usize {
        self.source_buffer.len()
    }

}