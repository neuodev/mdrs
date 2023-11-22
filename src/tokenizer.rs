use crate::bytes::CharIterator;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Hash(usize),
    Asterisk(usize),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    Whitespace,
    AngleBracket,
    Backticks(usize),
    Url(String),
    Dash,
    Underscore,
}

pub struct Tokenizer<'a> {
    chars: &'a mut CharIterator,
}

impl<'a> Tokenizer<'a> {
    pub fn new(chars: &'a mut CharIterator) -> Self {
        Self { chars }
    }

    pub fn consume_hashes(&mut self) -> Token {
        let mut count: usize = 0;
        while self.chars.current_char().char() == '#' {
            count += 1;
            self.chars.read_char();
        }

        Token::Hash(count)
    }

    pub fn consume_whitespace(&mut self) -> Token {
        while self.chars.current_char().char().is_whitespace() {
            self.chars.read_char();
        }

        Token::Whitespace
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bytes::Encoding;

    #[test]
    fn consume_hashes() {
        let mut chars = CharIterator::new();
        chars.read_from_str("#####", Some(Encoding::UTF8));
        let mut tokenizer = Tokenizer::new(&mut chars);

        assert_eq!(tokenizer.consume_hashes(), Token::Hash(5))
    }
}
