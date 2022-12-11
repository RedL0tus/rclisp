// use log::{debug, error, trace, warn};

use std::error::Error;
use std::fmt;

use crate::lexer::Token;
use crate::types::{cons, nil, quote, Object};

#[derive(Clone, Debug)]
pub enum ParserError {
    UnexpectedToken(Token),
    UnexpectedEOF,
    UnmatchedParens,
    EmptyInput,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::UnexpectedEOF => write!(f, "Unexpected EOF"),
            Self::UnexpectedToken(t) => write!(f, "{:?}", t),
            Self::UnmatchedParens => write!(f, "No matching parenthesis found"),
            Self::EmptyInput => write!(f, "Input is empty"),
        }
    }
}

impl Error for ParserError {}

impl From<Token> for Object {
    fn from(token: Token) -> Self {
        match token {
            Token::Integer(num) => Object::Integer(num),
            Token::Float(num) => Object::Float(num),
            Token::String(s) => Object::String(s),
            Token::Symbol(name) => {
                let n = name.to_uppercase();
                match n.as_str() {
                    "T" => Object::T,
                    "NIL" => nil(),
                    _ => {
                        Object::Symbol(n)
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

fn parse_quote<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Object, ParserError> {
    let next_token = tokens.next();
    if next_token == Some(Token::ParenLeft) {
        Ok(quote(parse_list(tokens)?))
    } else if let Some(n) = next_token {
        Ok(quote(n.into()))
    } else {
        Err(ParserError::UnexpectedEOF)
    }
}

fn parse_dotted_list<I: Iterator<Item = Token>>(
    last: Object,
    tokens: &mut I,
) -> Result<Object, ParserError> {
    let next_token = tokens.next();
    if next_token.is_none() {
        return Err(ParserError::UnexpectedEOF);
    }
    let next_token = next_token.unwrap();
    if [Token::ParenRight, Token::Dot].contains(&next_token) {
        return Err(ParserError::UnexpectedToken(next_token));
    }
    match next_token {
        Token::ParenRight | Token::Dot => Err(ParserError::UnexpectedToken(next_token)),
        Token::ParenLeft => Ok(cons(last, parse_list(tokens)?)),
        Token::Quote => Ok(cons(last, parse_quote(tokens)?)),
        _ => Ok(cons(last, next_token.into())),
    }
}

fn parse_list<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Object, ParserError> {
    let mut stack: Vec<Object> = vec![];
    let mut token = tokens.next();
    while let Some(t) = token {
        match t {
            Token::Dot => {
                if stack.is_empty() {
                    return Err(ParserError::UnexpectedToken(t));
                }
                let last = stack.pop().unwrap();
                stack.push(parse_dotted_list(last, tokens)?);
            }
            Token::ParenLeft => {
                stack.push(parse_list(tokens)?);
            }
            Token::Quote => {
                stack.push(parse_quote(tokens)?);
            }
            Token::ParenRight => {
                return Ok(Object::from(stack));
            }
            _ => {
                stack.push(t.into());
            }
        }
        token = tokens.next();
    }
    Err(ParserError::UnmatchedParens)
}

pub fn parse<I: Iterator<Item = Token>>(tokens: &mut I) -> Result<Object, ParserError> {
    let first_token = tokens.next();
    if first_token.is_none() {
        return Err(ParserError::EmptyInput);
    }
    let first_token = first_token.unwrap();
    match first_token {
        Token::Dot | Token::ParenRight => Err(ParserError::UnexpectedToken(first_token)),
        Token::ParenLeft => parse_list(tokens),
        Token::Quote => parse_quote(tokens),
        // _ => Ok(cons(Object::from(first_token), parse(tokens)?)),
        _ => Ok(first_token.into()),
    }
}

#[cfg(test)]
mod test {
    use super::super::lexer::Lexer;
    use super::parse;

    fn parse_and_compare<S1: AsRef<str>, S2: AsRef<str>>(orig: S1, res: S2) {
        let mut lexer = Lexer::new(orig.as_ref().as_bytes());
        let obj = parse(&mut lexer).unwrap();
        assert_eq!(res.as_ref(), obj.to_string());
    }

    #[test]
    fn test_parse() {
        parse_and_compare("'A", "'A");
        parse_and_compare("(test1 '(test2 . \"test3\")) ", "(TEST1 '(TEST2 . \"test3\"))");
        parse_and_compare("((A . B) . (C . D))", "((A . B) C . D)");
        parse_and_compare("(A)", "(A)");
    }
}
