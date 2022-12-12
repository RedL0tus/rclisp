//! Lexer

use log::{debug, trace};

use std::cmp;
use std::io::{BorrowedBuf, ErrorKind, Read, Result as IOResult};
use std::mem::MaybeUninit;

const DELIM: u8 = b' ';
const PAREN_LEFT: u8 = b'(';
const PAREN_RIGHT: u8 = b')';
const QUOTE: u8 = b'\'';
const STRING: u8 = b'"';
const ESCAPE: u8 = b'\\';
const DOT: u8 = b'.';
const PARENS: &[u8; 2] = &[PAREN_LEFT, PAREN_RIGHT];
const DELIMITERS: &[u8; 3] = &[DELIM, b'\n', b'\t'];
const DIGITS: &[u8; 10] = &[b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9'];
const NUM_CHARS: &[u8; 12] = &[
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'-', b'.',
];
const COMMENT: u8 = b';';
const COMMEND_END: u8 = b'\n';

pub const DEFAULT_BUF_SIZE: usize = if cfg!(target_os = "espidf") {
    512
} else {
    8 * 1024
};

macro_rules! token_from_buf {
    ($i:expr) => {
        if ($i).iter().all(|c| NUM_CHARS.contains(c)) && ($i).iter().any(|c| DIGITS.contains(c)) {
            trace!("{:?} is a number", $i);
            let has_dot = ($i).iter().any(|c| c == &DOT);
            let input_str = String::from_utf8_lossy($i).to_string();
            if has_dot {
                Token::Float(input_str.parse::<f64>().expect("Failed to parse to float"))
            } else {
                Token::Integer(input_str.parse::<isize>().expect("Failed to parse to integer"))
            }
        //     let mut negative = false;
        //     let mut met_digit = false;
        //     let (num, dec) = ($i).iter().fold((0, None), |(num, dec), c| {
        //         trace!("current value: {}, next char: {}", num, c);
        //         match *c {
        //             b'-' => {
        //                 if met_digit {
        //                     error!("Unexpected '-' in {:?}", $i);
        //                     panic!("Unexpected '-' in {:?}", $i);
        //                 }
        //                 trace!("Got a negative sign");
        //                 negative = !negative;
        //                 (num, dec)
        //             }
        //             b'.' => {
        //                 if dec.is_some() {
        //                     error!("Unexpected '.' in {:?}", $i);
        //                     panic!("Unexpected '.' in {:?}", $i);
        //                 }
        //                 trace!("Got a decimal point");
        //                 (num, Some(0))
        //             }
        //             _ => {
        //                 met_digit = true;
        //                 trace!("Next digit: {}", c);
        //                 if let Some(decimal) = dec {
        //                     (num, Some(decimal * 10 + (c - b'0') as isize))
        //                 } else {
        //                     (num * 10 + (c - b'0') as isize, dec)
        //                 }
        //             }
        //         }
        //     });
        //     if let Some(decimal) = dec {
        //         let mut d = decimal as f64;
        //         while d >= 1.0 {
        //             d /= 10.0;
        //         }
        //         let ret = num as f64 + d;
        //         Token::Float(if negative { -ret } else { ret })
        //     } else {
        //         Token::Integer(if negative { -num } else { num })
        //     }
        } else {
            let string = String::from_utf8_lossy($i).to_string();
            if string.starts_with(STRING as char) && string.ends_with(STRING as char) {
                Token::String(string[1..string.len() - 1].to_string())
            } else {
                Token::Symbol(string)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Integer(isize),
    Float(f64),
    Quote,
    Dot,
    Symbol(String),
    String(String),
    ParenLeft,
    ParenRight,
}

#[derive(Clone, Debug)]
pub struct Lexer<R> {
    inner: R,
    buf: Box<[MaybeUninit<u8>]>,
    pos: usize,
    filled: usize,
    initialized: usize,
}

impl<R> Lexer<R> {
    #[inline]
    fn get_buf(&self) -> &[u8] {
        unsafe { MaybeUninit::slice_assume_init_ref(self.buf.get_unchecked(self.pos..self.filled)) }
    }
}

impl<R: Read + std::fmt::Debug> Lexer<R> {
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: Box::new_uninit_slice(DEFAULT_BUF_SIZE),
            pos: 0,
            filled: 0,
            initialized: 0,
        }
    }

    #[inline]
    fn fill_buf(&mut self) -> IOResult<&[u8]> {
        if self.pos >= self.filled {
            let mut buf = BorrowedBuf::from(&mut *self.buf);
            unsafe {
                buf.set_init(self.initialized);
            }
            self.inner.read_buf(buf.unfilled())?;
            self.pos = 0;
            self.filled = buf.len();
            self.initialized = buf.init_len();
        }
        trace!("current status: {:?}", self);
        Ok(self.get_buf())
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.filled);
    }
}

impl<R: Read + std::fmt::Debug> Iterator for Lexer<R> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut token_buf = vec![];

        'read_loop: loop {
            let available = match self.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(_) => return None,
            };
            if available.is_empty() {
                break;
            }
            let mut i = 0usize;
            let mut in_string = false;
            let mut escape = false;
            let mut in_comment = false;
            while i < available.len() {
                let c = available[i];
                trace!("c: {}, i: {}, in_string: {}, escape: {}, token_buf: {:?}", c as char, i, in_string, escape, token_buf);
                if c == COMMENT {
                    in_comment = true;
                } else if (c == COMMEND_END) && in_comment {
                    in_comment = false;
                } else if in_comment {
                    // Do nothing
                } else if c == ESCAPE {
                    escape = !escape;
                    token_buf.push(c);
                } else if c == STRING {
                    token_buf.push(c);
                    if escape {
                        escape = false;
                    }
                    if in_string {
                        self.consume(i + 1);
                        break 'read_loop;
                    }
                    in_string = true;
                } else if in_string {
                    token_buf.push(c);
                } else if (PARENS.contains(&c) || c == QUOTE) && token_buf.is_empty() {
                    token_buf.push(c);
                    self.consume(i + 1);
                    break 'read_loop;
                } else if PARENS.contains(&c) {
                    self.consume(i);
                    break 'read_loop;
                } else if DELIMITERS.contains(&c) && (!token_buf.is_empty()) {
                    self.consume(i + 1);
                    break 'read_loop;
                } else if !DELIMITERS.contains(&c) {
                    token_buf.push(c);
                }
                i += 1;
            }
            self.consume(i);
        }

        let ret = if token_buf.is_empty() {
            None
        } else if token_buf.len() == 1 {
            match token_buf[0] {
                QUOTE => Some(Token::Quote),
                PAREN_LEFT => Some(Token::ParenLeft),
                PAREN_RIGHT => Some(Token::ParenRight),
                DOT => Some(Token::Dot),
                _ => Some(token_from_buf!(&token_buf)),
            }
        } else {
            Some(token_from_buf!(&token_buf))
        };
        debug!("Next token: {:?}", ret);
        ret
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn test_lexer() {
        let lexer = Lexer::new("(test1 (test2 test3)) ".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(
            result,
            vec![
                Token::ParenLeft,
                Token::Symbol("test1".to_string()),
                Token::ParenLeft,
                Token::Symbol("test2".to_string()),
                Token::Symbol("test3".to_string()),
                Token::ParenRight,
                Token::ParenRight
            ]
        );
    }

    #[test]
    fn test_lexer_malformed() {
        let lexer = Lexer::new("    (   'test1 (  test2 \n\t '(test3 . test4) )   ) ".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(
            result,
            vec![
                Token::ParenLeft,
                Token::Quote,
                Token::Symbol("test1".to_string()),
                Token::ParenLeft,
                Token::Symbol("test2".to_string()),
                Token::Quote,
                Token::ParenLeft,
                Token::Symbol("test3".to_string()),
                Token::Dot,
                Token::Symbol("test4".to_string()),
                Token::ParenRight,
                Token::ParenRight,
                Token::ParenRight,
            ]
        );
    }

    #[test]
    fn test_lexer_integer() {
        let lexer = Lexer::new("1".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Integer(1)]);
    }

    #[test]
    fn test_lexer_integer_negative() {
        let lexer = Lexer::new("-1".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Integer(-1)]);
    }

    #[test]
    #[should_panic]
    fn test_lexer_integer_negative_multiple() {
        let lexer = Lexer::new("----1".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Integer(-1)]);
    }

    #[test]
    #[should_panic]
    fn test_lexer_integer_negative_malformed() {
        let lexer = Lexer::new("--1-1".as_bytes());
        let _ = lexer.collect::<Vec<Token>>();
    }

    #[test]
    fn test_lexer_float() {
        let lexer = Lexer::new("1.0".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Float(1.0)]);
    }

    #[test]
    fn test_lexer_float_negative() {
        let lexer = Lexer::new("-1.0".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Float(-1.0)]);
    }

    #[test]
    #[should_panic]
    fn test_lexer_float_malformed() {
        let lexer = Lexer::new("--1.1.0".as_bytes());
        let _ = lexer.collect::<Vec<Token>>();
    }

    #[test]
    fn test_lexer_symbol() {
        let lexer = Lexer::new("-".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Symbol("-".to_string())]);
    }

    #[test]
    fn test_lexer_empty() {
        let lexer = Lexer::new("".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_lexer_parens() {
        let lexer = Lexer::new("()".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::ParenLeft, Token::ParenRight]);
    }

    #[test]
    fn test_lexer_comment() {
        let lexer = Lexer::new("; test \n test".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Symbol("test".to_string())]);
        let lexer = Lexer::new("; test\n;another test\ntest ;test".as_bytes());
        let result = lexer.collect::<Vec<Token>>();
        assert_eq!(result, vec![Token::Symbol("test".to_string())]);
    }
}
