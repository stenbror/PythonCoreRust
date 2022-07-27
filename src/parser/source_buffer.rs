
pub struct SourceBuffer {
    source_buffer: Box<Vec<char>>,
    index: u32
}

impl SourceBuffer {

    /// Construct new source buffer from given string containing sourcefile from file, evalinput or console input
    pub fn new(buffer: String) -> SourceBuffer {
        SourceBuffer {
            source_buffer: Box::new(buffer.chars().collect()),
            index: 0
        }
    }

    /// Returns current character at position into source buffer, does not advance by itself, so we can read it many times if needed
    pub fn get_char(&self) -> &char {
        let index_local = *&self.index as usize;
        let max = *&self.source_buffer.len() as usize;
        if (index_local < max) { &*&(self.source_buffer[index_local]) } else { &'\0' }
    }

    /// Returns a peek into current position and next two position in source buffer, without any advance. At end it will just return eof as character.
    pub fn peek_three_chars(&self) -> ( &char, &char, &char ) {
        let index_local = *&self.index as usize;
        let max = *&self.source_buffer.len() as usize;
        let a = if (index_local < max) { &*&(self.source_buffer[index_local]) } else { &'\0' };
        let b = if ((index_local + 1) < max) { &*&(self.source_buffer[index_local + 1]) } else { &'\0' };
        let c = if ((index_local + 2) < max) { &*&(self.source_buffer[index_local + 2]) } else { &'\0' };
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

#[cfg(test)]
mod tests {
    use crate::parser::source_buffer::SourceBuffer;

    #[test]
    fn read_characters_from_empty_source_buffer() {
        let mut buffer = Box::new( SourceBuffer::new("".to_string()) );
        assert_eq!(buffer.get_position(), &0u32);
        assert_eq!(buffer.get_char(), &'\0');
    }

    #[test]
    fn read_characters_from_source_buffer() {
        let mut buffer = Box::new( SourceBuffer::new("def a(): pass".to_string()) );
        assert_eq!(buffer.get_position(), &0u32);
        assert_eq!(buffer.get_char(), &'d');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'e');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'f');
        buffer.advance();
        assert_eq!(buffer.get_char(), &' ');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'a');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'(');
        buffer.advance();
        assert_eq!(buffer.get_char(), &')');
        buffer.advance();
        assert_eq!(buffer.get_char(), &':');
        buffer.advance();
        assert_eq!(buffer.get_char(), &' ');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'p');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'a');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'s');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'s');
        buffer.advance();
        assert_eq!(buffer.get_char(), &'\0');
    }

    #[test]
    fn peek_next_three_characters_and_check_position() {
        let mut buffer = Box::new( SourceBuffer::new("def a(): pass".to_string()) );
        let ( a, b, c ) = buffer.peek_three_chars();
        assert_eq!(a, &'d');
        assert_eq!(b, &'e');
        assert_eq!(c, &'f');
        assert_eq!(buffer.get_position(), &0u32);
    }

    #[test]
    fn length_of_source_buffer_content() {
        let mut buffer = Box::new( SourceBuffer::new("def a(): pass".to_string()) );
        assert_eq!(buffer.count(), (13u32 as usize));
    }

    #[test]
    fn peek_next_three_characters_on_buffer_withy_zero_character_only() {
        let mut buffer = Box::new( SourceBuffer::new("".to_string()) );
        let ( a, b, c ) = buffer.peek_three_chars();
        assert_eq!(a, &'\0');
        assert_eq!(b, &'\0');
        assert_eq!(c, &'\0');
        assert_eq!(buffer.get_position(), &0u32);
    }

    #[test]
    fn peek_next_three_characters_on_buffer_withy_one_character_only() {
        let mut buffer = Box::new( SourceBuffer::new("d".to_string()) );
        let ( a, b, c ) = buffer.peek_three_chars();
        assert_eq!(a, &'d');
        assert_eq!(b, &'\0');
        assert_eq!(c, &'\0');
        assert_eq!(buffer.get_position(), &0u32);
    }

    #[test]
    fn peek_next_three_characters_on_buffer_withy_two_character_only() {
        let mut buffer = Box::new( SourceBuffer::new("de".to_string()) );
        let ( a, b, c ) = buffer.peek_three_chars();
        assert_eq!(a, &'d');
        assert_eq!(b, &'e');
        assert_eq!(c, &'\0');
        assert_eq!(buffer.get_position(), &0u32);
    }

    #[test]
    fn peek_next_three_characters_on_buffer_withy_three_character_only() {
        let mut buffer = Box::new( SourceBuffer::new("def".to_string()) );
        let ( a, b, c ) = buffer.peek_three_chars();
        assert_eq!(a, &'d');
        assert_eq!(b, &'e');
        assert_eq!(c, &'f');
        assert_eq!(buffer.get_position(), &0u32);
    }
}
