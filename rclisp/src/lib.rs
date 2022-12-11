#![feature(read_buf)]
#![feature(new_uninit)]
#![feature(maybe_uninit_slice)]

pub mod env;
pub mod lexer;
pub mod parse;
pub mod types;
pub mod eval;
pub mod builtins;

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
