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
    SingleQuote,
    DoubleQuote,
    Space
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Text(s) => write!(f, "{}", s),
            Token::Pipe => write!(f, "|"),
            Token::RedirectOutput => write!(f, ">"),
            Token::RedirectInput => write!(f, "<"),
            Token::Background => write!(f, "&"),
            Token::SingleQuote => write!(f, "'"),
            Token::DoubleQuote => write!(f, "\""),
            Token::Space => write!(f, " "),
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
            "'" => Token::SingleQuote,
            "\"" => Token::DoubleQuote,
            " " => Token::Space,
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

    pub fn skip_whitespace(&mut self) {
        self.input = self.input.trim_start();
    }

    pub fn is_empty(&self) -> bool {
      self.input.is_empty()
    }
    
    pub fn peek_next(&mut self) -> Option<Token> {
      self.parse_next_token(false)
    }
    
    pub fn next_token(&mut self) -> Option<Token> {
      self.parse_next_token(true)
    }

    fn is_special_token(&self, c: char) -> bool {
        match c {
            '|' | '>' | '<' | '&' | '"' | '\'' | ' '  => true,
            _ => false,
        }
    }

    fn parse_next_token(&mut self, advance_stream: bool) -> Option<Token> {
        if self.input.is_empty() {
            return None;
        }

        if let Some(c) = self.input.chars().next() {
          if self.is_special_token(c) {
            return self.special_token(advance_stream);
          } else {
            return self.text_token(advance_stream);
          }
        }

        None
    }

    fn special_token(&mut self, advance_stream: bool) -> Option<Token> {
        if let Some(c) = self.input.chars().next() {
          let end = c.len_utf8();
          let toke = Token::new(&self.input[0..end]);

          if advance_stream {
            self.input = &self.input[end..];
          }

          return Some(toke);
        }

        None
    }

    fn text_token(&mut self, advance_stream: bool) -> Option<Token> {
      let mut end = 0;
      for (idx, c) in self.input.char_indices() {
        if self.is_special_token(c) { 
          end = idx;
          break 
        };

        end = idx + c.len_utf8();
      }

      if end > 0 {
        let toke = Token::new(&self.input[0..end]);
        if advance_stream {
          self.input = &self.input[end..];
        };
        return Some(toke);
      }

      None
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
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_2() {
        let input = "ls -l | grep .rs | wc -l";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("wc")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_3() {
        let input = "ls -l | grep .rs | wc -l &";
        let mut tokenizer = Tokenizer::new(input);
  
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("wc")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Background));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_tokenizer_4() {
        let input = "ls -l | grep .rs | wc -l & echo \"hello world\"";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next_token(), Some(Token::Text("ls")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("grep")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".rs")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Pipe));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("wc")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("-l")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Background));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("echo")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::DoubleQuote));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("hello")));
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text("world")));
        assert_eq!(tokenizer.next_token(), Some(Token::DoubleQuote));
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
        assert_eq!(tokenizer.next_token(), Some(Token::Space));
        assert_eq!(tokenizer.next_token(), Some(Token::Text(".txt")));
        assert_eq!(tokenizer.next_token(), None);
    }
}