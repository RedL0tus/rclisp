#![feature(read_buf)]
#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]

pub mod env;
pub mod lexer;
pub mod parse;
pub mod types;
pub mod eval;
pub mod builtins;

use log::debug;

use std::fmt;
use std::io::Read;

pub use lexer::Lexer;
pub use eval::{Eval, EvalError};
pub use types::Object;
pub use env::RcEnv;
pub use builtins::generate_default_env;

// use eval::Eval;

// const PROGRAM: &str = r#"(defun my-assoc (v alist)
// (cond ((null alist) nil)
//       ((eq (car (car alist)) v) (car alist))
//       (t (my-assoc v (cdr alist)))))
      
//       (my-assoc 'F '((A . B)(C E F)(B)))"#;

// fn main() {
//     let env = builtins::generate_default_env();
//     // println!("Env: {:#?}", env);
//     // let mut lexer = lexer::Lexer::new("(funcall (lambda (x y) (+ x y)) (1 2))".as_bytes());
//     let mut lexer = lexer::Lexer::new(PROGRAM.as_bytes());
//     // let mut lexer = lexer::Lexer::new("(+ (+ 1 2) (+ 3 4))".as_bytes());
//     while let Ok(obj) = parse::parse(&mut lexer) {
//         println!("Parse Result: {:?}", obj);
//         println!("Result: {:?}", obj.eval(&env).unwrap());
//     }
//     // println!("Env: {:?}", env);
// }

pub fn interpret<R: Read + fmt::Debug>(source: R, env: &RcEnv) -> Result<Object, EvalError> {
    let mut lexer = Lexer::new(source);
    let mut ret = Ok(Object::Nil);
    while let Ok(obj) = parse::parse(&mut lexer) {
        debug!("parse result: {} {:?}", obj, obj);
        ret = obj.eval(&env);
        debug!("evaluation result: {:?}", ret);
    }
    ret
}
