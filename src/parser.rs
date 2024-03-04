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
                Token::Text(_) => {
                    args.push(self.parse_argument()?);
                },
                _ => break,
            }
        }

        Ok(RshNode::Command { name, args })
    }

    fn parse_argument(&mut self) -> Result<String, ParseError> {
        match self.tokenizer.next_token() {
            Some(Token::Text(text)) => Ok(text.to_string()),
            Some(token) => Err(ParseError::UnexpectedToken(format!("{:?}", token))),
            None => Err(ParseError::UnexpectedEOF),
        }
    }
}


#[cfg(test)]
mod integration {
    use super::*;

    #[test]
    fn test_parser_vanilla() {
        let input = "echo \"hello world\"";
        let mut parser = Parser::new(input);
        let result = parser.parse();
        assert!(result.is_ok());
        let unwrapped = result.unwrap();
        assert_eq!(unwrapped, RshNode::Command {
            name: "echo".to_string(),
            args: vec!["\"hello world\"".to_string()],
        });
    }

    #[test]
    fn test_parser_pipe() {
        let input = "ls -l | grep .rs";
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
        assert!(result.is_ok());
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

