use rustyline::validate::MatchingBracketValidator;
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
pub struct RCLReadlineHelper {
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
}

impl RCLReadlineHelper {
    pub fn new() -> Self {
        Self {
            validator: MatchingBracketValidator::new(),
        }
    }
}
