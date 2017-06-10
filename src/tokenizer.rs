use std::path::Path;

use {Result, Token, Position};
use tokens;
use values;

/// Tokenizer.
///
/// This is an iterator which tokenizes Erlang source code and iterates on the resulting tokens.
///
/// # Examples
///
/// ```
/// use erl_tokenize::{Tokenizer, TokenKind};
///
/// let src = r#"io:format("Hello")."#;
/// let tokens = Tokenizer::new(src).collect::<Result<Vec<_>, _>>().unwrap();
///
/// assert_eq!(tokens.iter().map(|t| t.text()).collect::<Vec<_>>(),
///            ["io", ":", "format", "(", r#""Hello""#, ")", "."]);
/// ```
#[derive(Debug)]
pub struct Tokenizer<'a> {
    text: &'a str,
    next_pos: Position,
}
impl<'a> Tokenizer<'a> {
    /// Makes a new `Tokenizer` instance which tokenize the Erlang source code text.
    pub fn new(text: &'a str) -> Self {
        let init_pos = Position::new();
        Tokenizer {
            text,
            next_pos: init_pos.clone(),
        }
    }

    /// Sets the file path of the succeeding tokens.
    pub fn set_filepath<P: AsRef<Path>>(&mut self, filepath: P) {
        self.next_pos.set_filepath(filepath);
    }

    /// Returns the input text.
    pub fn text(&self) -> &'a str {
        self.text
    }

    /// Returns the cursor position from which this tokenizer will start to scan the next token.
    ///
    /// # Examples
    ///
    /// ```
    /// use erl_tokenize::Tokenizer;
    ///
    /// let src = r#"io:format(
    ///   "Hello")."#;
    ///
    /// let mut tokenizer = Tokenizer::new(src);
    /// assert_eq!(tokenizer.next_position().offset(), 0);
    ///
    /// assert_eq!(tokenizer.next().unwrap().map(|t| t.text().to_owned()).unwrap(), "io");
    /// assert_eq!(tokenizer.next_position().offset(), 2);
    /// tokenizer.next(); // ':'
    /// tokenizer.next(); // 'format'
    /// tokenizer.next(); // '('
    /// tokenizer.next(); // '\n'
    /// assert_eq!(tokenizer.next_position().offset(), 11);
    /// assert_eq!(tokenizer.next_position().line(), 2);
    /// assert_eq!(tokenizer.next_position().column(), 1);
    /// assert_eq!(tokenizer.next().unwrap().map(|t| t.text().to_owned()).unwrap(), " ");
    /// assert_eq!(tokenizer.next_position().offset(), 12);
    /// assert_eq!(tokenizer.next_position().line(), 2);
    /// assert_eq!(tokenizer.next_position().column(), 2);
    /// ```
    pub fn next_position(&self) -> Position {
        self.next_pos.clone()
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos.offset() >= self.text.len() {
            None
        } else {
            let text = unsafe {
                self.text
                    .slice_unchecked(self.next_pos.offset(), self.text.len())
            };
            let cur_pos = self.next_pos.clone();
            match track!(Token::from_text(text, cur_pos)) {
                Err(e) => Some(Err(e)),
                Ok(t) => {
                    match t {
                        Token::Whitespace(ref v @ tokens::WhitespaceToken { .. })
                            if v.value() ==
                               values::Whitespace::Newline => {
                            self.next_pos.new_line();
                        }
                        _ => {
                            self.next_pos.step(t.text().len());
                        }
                    }
                    Some(Ok(t))
                }
            }
        }
    }
}
