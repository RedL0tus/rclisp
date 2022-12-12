mod helper;

use log::{debug, error};
use clap::Parser;
use anyhow::{Error, bail};
use rustyline::Editor;

use rclisp::{interpret, generate_default_env};

use std::path::PathBuf;
use std::env;
use std::fs;

use helper::RCLReadlineHelper;

const BANNER: &str = r#"
This is rcl {version}, really crappy lisp interpreter.
An incomplete implementation of Common Lisp.
GitHub: <https://github.com/RedL0tus/rclisp>

rcl is free software, provided as is, with absolutely no warranty.

{vars} variables registered.
"#;

#[derive(Clone, Debug, Parser)]
#[command(author, version, about, long_about = "Really Crappy Lisp Interpreter")]
struct Args {
    /// Load file into REPL environment
    #[arg(short, long, value_name = "FILE")]
    pub load: Vec<PathBuf>,
    /// Evaluate file
    #[arg(short, long, value_name = "FILE")]
    pub eval: Vec<PathBuf>,
    /// Enable debug output
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
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

    // Initial global env
    let env = generate_default_env();

    // Load files
    for path in args.load {
        if let Ok(file) = fs::File::open(&path) {
            interpret(file, &env)?;
        } else {
            error!("Failed to open file: {:?}", &path);
        }
    }

    if !args.eval.is_empty() {
        for path in args.eval {
            if let Ok(file) = fs::File::open(&path) {
                interpret(file, &env)?;
            } else {
                error!("Failed to evaluate file: {:?}", &path);
                bail!("Failed to evaluate file: {:?}", &path);
            }
        }
        return Ok(());
    }

    // Editor
    let mut editor = Editor::new()?;
    editor.set_helper(Some(RCLReadlineHelper::new()));

    let banner = BANNER.replace("{version}", env!("CARGO_PKG_VERSION"));
    let banner = banner.replace("{vars}", &env.borrow().len().to_string());

    println!("{}", banner);

    loop {
        let input = editor.readline("* ")?;
        match interpret(input.as_bytes(), &env) {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("Encountered error: {}", e),
        }
    }
}
