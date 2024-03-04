use std::fmt::{self, Display, Formatter};
/*
Basic tokens that the shell program can handle...
*/
#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Text(&'src str),
    Pipe,
    RedirectOutput,
    RedirectInput,
    Background,
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(s) => write!(f, "{}", s),
            Token::Pipe => write!(f, "|"),
            Token::RedirectOutput => write!(f, ">"),
            Token::RedirectInput => write!(f, "<"),
            Token::Background => write!(f, "&"),
        }
    }
}

impl<'src> Token<'src> {
    pub fn new(token: &'src str) -> Token {
        match token {
            "|" => Token::Pipe,
            ">" => Token::RedirectOutput,
            "<" => Token::RedirectInput,
            "&" => Token::Background,
            _ => Token::Text(token),
        }
    }
}

/*
The tokenizer that takes a string and then will break it into tokens.
*/
#[derive(Debug)]
pub struct Tokenizer<'src> {
    input: &'src str,
}

impl<'src> Tokenizer<'src> {
    pub fn new(input: &'src str) -> Tokenizer<'src> {
        Tokenizer {
            input: input,
        }
    }

    fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    fn is_special_char(&self, c: char) -> bool {
        match c {
            '|' | '>' | '<' | '&' => true,
            _ => false,
        }
    }
    
    pub fn peek_next(&mut self) -> Option<Token> {
        self.parse_next_token(false)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.parse_next_token(true)
    }

    fn parse_next_token(&mut self, advance_stream: bool) -> Option<Token> {
        if self.input.is_empty() {
            return None;
        }
    
        self.skip_whitespace();
        
        // Check for a special character at the beginning and handle it.
        if let Some(first_char) = self.input.chars().next() {
            if self.is_special_char(first_char) {
                let toke = Token::new(&self.input[..first_char.len_utf8()]);
                if advance_stream {
                    self.input = &self.input[first_char.len_utf8()..];
                }
                return Some(toke);
            }
        }
    
        let iter = self.input.char_indices();
        let mut end = 0;
        let mut in_quote = false;
    
        for (start, c) in iter {
            if c == '"' {
                in_quote = !in_quote;
            } else if !in_quote && (self.is_special_char(c) || c.is_whitespace()) {
                // Since we're already handling special characters at the beginning,
                // encountering one here means we're at the end of a text token.
                break;
            }
            end = start + c.len_utf8();
        }
    
        let token = Token::new(&self.input[..end]);
    
        if advance_stream {
            self.input = &self.input[end..];
        }
    
        Some(token)
    }
}


// unit tests
#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let input = "ls -l | grep .rs";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_2() {
        let input = "ls -l | grep .rs | wc -l";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("wc")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_3() {
        let input = "ls -l | grep .rs | wc -l &";
        let mut tokenizer = Tokenizer::new(input);
  
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("wc")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Background));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_4() {
        let input = "ls -l | grep .rs | wc -l & echo \"hello world\"";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("wc")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Background));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("echo")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("\"hello world\"")));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_tight_syntax() {
        let input = "ls>out.txt";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::RedirectOutput));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("out.txt"))); 
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_tight_syntax_2() {
        let input = "ls>out.txt|grep .txt";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::RedirectOutput));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("out.txt"))); 
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".txt")));
        assert_eq!(tokenizer.next_token(), None);
    }
}