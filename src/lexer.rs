// use crate::token::Token;


#[derive(Default)]
pub struct Lexer {
    pub input : String,
    pub position : usize,
    pub next_position : usize,
    pub ch : char,

}

impl Lexer {
    pub fn read_char(&mut self)
    {
        if(self.next_position > self.input.len())
        {
            self.ch = '0';
        } else {
            //as only ASCII text is handled
            self.ch = self.input.as_bytes()[self.next_position] as char;
        }
        self.position = self.next_position;
        self.next_position += 1;
    } 
}