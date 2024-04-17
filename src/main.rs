
use trashlang::lexer;

fn main()
{
    let mut lexi = lexer::Lexer{input : String::from("let fn = x"), position : 0, next_position : 1, ch : '0'};
    lexi.read_char();
}
