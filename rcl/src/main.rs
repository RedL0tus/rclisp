mod helper;

use log::debug;
use clap::Parser;
use anyhow::Error;
use rustyline::Editor;

use rclisp::lexer::Lexer;
use rclisp::parse::parse;
use rclisp::eval::Eval;
use rclisp::builtins::generate_default_env;

use std::path::PathBuf;
use std::env;

use helper::RCLReadlineHelper;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = "Really Crappy Lisp Interpreter")]
struct Args {
    /// Load file into REPL environment
    #[arg(short, long, value_name = "FILE")]
    load: Vec<PathBuf>,
    /// Evaluate file
    #[arg(short, long, value_name = "FILE")]
    eval: Vec<PathBuf>,
    /// Enable debug output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    // Setup logger
    if env::var("RCL_LOG").is_err() {
        let level = match args.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        };
        env::set_var("RCL_LOG", level);
    }
    pretty_env_logger::try_init_custom_env("RCL_LOG")?;

    debug!("args: {:?}", args);

    // Editor
    let mut editor = Editor::new()?;
    editor.set_helper(Some(RCLReadlineHelper::new()));

    let env = generate_default_env();
    loop {
        let input = editor.readline("* ")?;
        let mut lexer = Lexer::new(input.as_bytes());
        // let lexer: Vec<rclisp::lexer::Token> = lexer.collect();
        // println!("lexer: {:?}", lexer);
        // let mut lexer = lexer.into_iter();
        while let Ok(obj) = parse(&mut lexer) {
            debug!("Parse Result: {:?}", obj);
            println!("{}", obj.eval(&env).unwrap().print());
        }
    }
}
