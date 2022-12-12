use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use xterm_js_rs::addons::fit::FitAddon;
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};

use rclisp::{generate_default_env, interpret};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const PROMPT: &str = "* ";
const PROMPT_CONTINUE: &str = "  ";
const ESCAPE: char = '\\';

fn prompt(term: &Terminal, cont: bool) {
    term.writeln("");
    let prompt = if cont {
        PROMPT_CONTINUE
    } else {
        PROMPT
    };
    term.write(prompt);
}

fn quotes_matched<S: AsRef<str>>(input: S) -> bool {
    let mut escape = false;
    let mut count = 0;
    for c in input.as_ref().chars() {
        match (c, escape) {
            (ESCAPE, _) => escape = !escape,
            ('(', false) => count += 1,
            (')', false) => count -= 1,
            (_, true) => escape = false,
            (_, false) => (),
        }
    }
    count == 0
} 

// Keyboard keys
// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_C: u32 = 67;
const KEY_L: u32 = 76;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let terminal: Terminal = Terminal::new(
        TerminalOptions::new()
            .with_rows(64)
            .with_cursor_blink(true)
            .with_cursor_width(10)
            .with_font_size(16)
            .with_draw_bold_text_in_bright_colors(true)
            .with_right_click_selects_word(true)
            .with_theme(
                Theme::new()
                    .with_foreground("#FFFFFF")
                    .with_background("#000000"),
            ),
    );

    let elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap();

    terminal.writeln(&format!("rcl-wasm {}, really crappy lisp interpreter", env!("CARGO_PKG_VERSION")));
    terminal.writeln(&format!("An incomplete implementation of Common Lisp, this time in browsers."));
    terminal.writeln("");
    terminal.writeln(&format!("rcl-wasm is free software, provided as is, with absolutely no warranty."));
    terminal.writeln("");
    terminal.writeln("WARNING: print, princ, and terpri currently don't work");
    terminal.open(elem.dyn_into()?);
    prompt(&terminal, false);

    let mut line = String::new();
    let mut cursor_col = 0;

    let env = generate_default_env();

    let term: Terminal = terminal.clone().dyn_into()?;

    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let event = e.dom_event();
        match event.key_code() {
            KEY_ENTER => {
                if line.is_empty() {
                    prompt(&term, false);
                } else if quotes_matched(&line) {
                    term.writeln("");
                    match interpret(line.as_bytes(), &env) {
                        Ok(res) => term.writeln(&format!("{}", res)),
                        Err(e) => term.writeln(&format!("Error: {}", e)),
                    }
                    line.clear();
                    prompt(&term, false);
                } else {
                    prompt(&term, true);
                    line.push_str(" ");
                }
                cursor_col = 0;
            }
            KEY_BACKSPACE => {
                if cursor_col > 0 {
                    term.write("\u{0008} \u{0008}");
                    line.pop();
                    cursor_col -= 1;
                }
            }
            KEY_LEFT_ARROW => {
                if cursor_col > 0 {
                    term.write(CURSOR_LEFT);
                    cursor_col -= 1;
                }
            }
            KEY_RIGHT_ARROW => {
                if cursor_col < line.len() {
                    term.write(CURSOR_RIGHT);
                    cursor_col += 1;
                }
            }
            KEY_L if event.ctrl_key() => term.clear(),
            KEY_C if event.ctrl_key() => {
                prompt(&term, false);
                line.clear();
                cursor_col = 0;
            }
            _ => {
                if !event.alt_key() && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
                    term.write(&event.key());
                    line.push_str(&e.key());
                    cursor_col += 1;
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    terminal.on_key(callback.as_ref().unchecked_ref());

    callback.forget();

    let addon = FitAddon::new();
    terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
    addon.fit();
    terminal.focus();

    Ok(())
}
