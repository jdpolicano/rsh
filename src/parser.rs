use crate::token::{ Tokenizer, Token };

// error types for syntax parsing...
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    UnexpectedEOF,
    UnexpectedChar(char),
    UnexpectedEOL,
}

// result type for parsing 
pub type ParseResult = Result<RshNode, ParseError>;

// redirect mode controls how the redirect will be handled.
#[derive(Debug, PartialEq)]
pub enum RedirectMode {
    Read,
    Write,
    Append, // not sure we need this yet...
}

#[derive(Debug, PartialEq)]
pub enum RshNode {
    Command {
        name: String,
        args: Vec<String>,
    },
    Pipe {
        left: Box<RshNode>,
        right: Box<RshNode>,
    },
    Redirect {
        command: Box<RshNode>,
        file: String,
        mode: RedirectMode,
    },
    Background {
        command: Box<RshNode>,
    },
}

impl RshNode {
    pub fn is_background(&self) -> bool {
        match self {
            RshNode::Background { .. } => true,
            _ => false,
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match self {
            RshNode::Command { name, .. } =>Some(name),
            RshNode::Redirect { command, .. } => command.get_name(),
            RshNode::Background { command } => command.get_name(),
            _ => None,
        }
    }

    pub fn get_args(&self) -> Option<&[String]> {
        match self {
            RshNode::Command { args, .. } => Some(args),
            RshNode::Redirect { command, .. } => command.get_args(),
            RshNode::Background { command } => command.get_args(),
            _ => None,
        }
    }
}

pub struct Parser<'src> {
    tokenizer: Tokenizer<'src>,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Parser<'src> {
        Parser {
            tokenizer: Tokenizer::new(input),
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.parse_command()
    }

    fn parse_command(&mut self) -> ParseResult {
        let mut command = self.parse_simple_command()?;

        while let Some(token) = self.tokenizer.peek_next() {
            match token {
                Token::Pipe => {
                    self.tokenizer.next_token();
                    let right = self.parse_command()?;
                    command = RshNode::Pipe {
                        left: Box::new(command),
                        right: Box::new(right),
                    };
                },
                Token::RedirectOutput => {
                    self.tokenizer.next_token();
                    let file = self.parse_argument()?;
                    command = RshNode::Redirect {
                        command: Box::new(command),
                        file,
                        mode: RedirectMode::Write,
                    };
                },
                Token::RedirectInput => {
                    self.tokenizer.next_token();
                    let file = self.parse_argument()?;
                    command = RshNode::Redirect {
                        command: Box::new(command),
                        file,
                        mode: RedirectMode::Read,
                    };
                },
                Token::Background => {
                    self.tokenizer.next_token();
                    command = RshNode::Background {
                        command: Box::new(command),
                    };
                },
                _ => break,
            }
        }

        Ok(command)
    }

    fn parse_simple_command(&mut self) -> ParseResult {
        let name = self.parse_argument()?;
        let mut args = Vec::new();

        while let Some(token) = self.tokenizer.peek_next() {
            match token {
                Token::Text(_) | Token::SingleQuote | Token::DoubleQuote => {
                    args.push(self.parse_argument()?);
                },
                Token::Space => { self.skip_whitespace() }
                _ => break,
            }
        }

        Ok(RshNode::Command { name, args })
    }

    fn parse_argument(&mut self) -> Result<String, ParseError> {
        self.skip_whitespace();

        if self.next_is(Token::SingleQuote) {
          return self.parse_until_next(Token::SingleQuote);
        }

        if self.next_is(Token::DoubleQuote){
          return self.parse_until_next(Token::DoubleQuote);
        }

        if let Some(token) = self.tokenizer.next_token() {
            match token {
              Token::Text(t) => { 
                return Ok(t.to_string());
              },
              _ => { return Err(ParseError::UnexpectedToken(token.to_string())) }
            }
        }

        Err(ParseError::UnexpectedEOF)
    }

    // collects stuff into a string until it hits the next token, "token".
    fn parse_until_next(&mut self, token: Token) -> Result<String, ParseError> {
        self.tokenizer.next_token();

        if self.tokenizer.is_empty() {
          return Err(ParseError::UnexpectedEOF);
        }

        let mut res = String::new();
        while let Some(t) = self.tokenizer.next_token() {
          if t == token { break; };
          res.push_str(&t.to_string());
        }

        Ok(res)
    }

    fn next_is(&mut self, token: Token) -> bool {
      if let Some(t) = self.tokenizer.peek_next() {
        return t == token;
      }
      return false;
    }

    fn skip_whitespace(&mut self) {
        self.tokenizer.skip_whitespace();
    }
}


#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn test_parser_vanilla() {
        let input = "echo hello";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Command {
            name: "echo".to_string(),
            args: vec!["hello".to_string()],
        });
    }

    #[test]
    fn test_parser_vanilla_double_quote() {
        let input = "echo \"hello world\"";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Command {
            name: "echo".to_string(),
            args: vec!["hello world".to_string()],
        });
    }

    #[test]
    fn test_parser_vanilla_single_quote() {
        let input = "echo 'hello world'";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Command {
            name: "echo".to_string(),
            args: vec!["hello world".to_string()],
        });
    }

    #[test]
    fn test_parser_pipe() {
        let input = "ls -l|grep .rs";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Pipe {
            left: Box::new(RshNode::Command {
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            }),
            right: Box::new(RshNode::Command {
                name: "grep".to_string(),
                args: vec![".rs".to_string()],
            }),
        });
    }

    #[test]
    fn test_parser_pipe_multiple() {
        let input = "ls -l | grep .rs | wc -l";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        if !result.is_ok() {
          println!("{:?}", result);
          assert!(false);
        }
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Pipe {
            left: Box::new(RshNode::Command {
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            }),
            right: Box::new(RshNode::Pipe {
                left: Box::new(RshNode::Command {
                    name: "grep".to_string(),
                    args: vec![".rs".to_string()],
                }),
                right: Box::new(RshNode::Command {
                    name: "wc".to_string(),
                    args: vec!["-l".to_string()],
                }),
            }),
        });
    }

    #[test]
    fn test_parser_pipe_multiple_background() {
        let input = "ls -l|grep .rs|wc -l&";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();

        // a | b | c  

        assert_eq!(unwrapped, RshNode::Pipe {
            left: Box::new(RshNode::Command {
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            }),
            right: Box::new(RshNode::Pipe {
                left: Box::new(RshNode::Command {
                    name: "grep".to_string(),
                    args: vec![".rs".to_string()],
                }),
                right: Box::new(RshNode::Background {
                    command: Box::new(RshNode::Command {
                        name: "wc".to_string(),
                        args: vec!["-l".to_string()],
                    }),
                }),
            })
        });
    }

    #[test]
    fn test_parser_pipe_multiple_background_deep() {
        let input = "ls -l|grep .rs|wc -l&|something|something else";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();

        // a | b | c
        assert_eq!(unwrapped, RshNode::Pipe {
            left: Box::new(RshNode::Command {
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            }),
            right: Box::new(RshNode::Pipe {
                left: Box::new(RshNode::Command {
                    name: "grep".to_string(),
                    args: vec![".rs".to_string()],
                }),
                right: Box::new(RshNode::Pipe {
                    left: Box::new(RshNode::Background {
                        command: Box::new(RshNode::Command {
                            name: "wc".to_string(),
                            args: vec!["-l".to_string()],
                        }),
                    }),
                    right: Box::new(RshNode::Pipe {
                        left: Box::new(RshNode::Command {
                            name: "something".to_string(),
                            args: vec![],
                        }),
                        right: Box::new(RshNode::Command {
                            name: "something".to_string(),
                            args: vec!["else".to_string()],
                        }),
                    }),
                }),
            }),
        });
    }

    #[test]
    fn test_parser_redirec_to_file() {
        let input = "ls -l > dir.txt";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Redirect {
            command: Box::new(RshNode::Command {
                name: "ls".to_string(),
                args: vec!["-l".to_string()],
            }),
            file: "dir.txt".to_string(),
            mode: RedirectMode::Write,
        });
    }

    #[test]
    fn test_parser_redirec_from_file() {
        let input = "cat < dir.txt";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Redirect {
            command: Box::new(RshNode::Command {
                name: "cat".to_string(),
                args: vec![],
            }),
            file: "dir.txt".to_string(),
            mode: RedirectMode::Read,
        });
    }
}

