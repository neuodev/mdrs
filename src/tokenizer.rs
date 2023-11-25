use crate::bytes::{Bytes, CharIterator};
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    String(String),
    Hash(usize),
    Asterisk(usize),
    Backticks(usize),
    Dash(usize),
    Underscore(usize),
    Url(String),
    Whitespace(String),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    AngleBracket,
    ExclamationMark,
    EOF,
}

impl Token {
    pub fn is_string(&self) -> bool {
        matches!(self, Token::String(..))
    }

    pub fn is_hash(&self) -> bool {
        matches!(self, Token::Hash(..))
    }

    pub fn is_asterisk(&self) -> bool {
        matches!(self, Token::Asterisk(..))
    }

    pub fn is_backticks(&self) -> bool {
        matches!(self, Token::Backticks(..))
    }

    pub fn is_dash(&self) -> bool {
        matches!(self, Token::Dash(..))
    }

    pub fn is_underscore(&self) -> bool {
        matches!(self, Token::Underscore(..))
    }

    pub fn is_url(&self) -> bool {
        matches!(self, Token::Url(..))
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, Token::Whitespace(..))
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Token::EOF)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct ParseTokenError;

impl FromStr for Token {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Ok(Token::EOF);
        }

        let chars = s.chars().collect::<Vec<char>>();
        let char = chars.first().unwrap().clone();

        let token = match char {
            '(' => Token::OpeningParenthesis,
            ')' => Token::ClosingParenthesis,
            '[' => Token::OpeningBracket,
            ']' => Token::ClosingBracket,
            '>' => Token::AngleBracket,
            '!' => Token::ExclamationMark,
            _ => todo!(),
        };

        Ok(token)
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::String(s) => s.clone(),
            Token::Hash(n) => "#".repeat(*n),
            Token::Asterisk(n) => "*".repeat(*n),
            Token::Backticks(n) => "`".repeat(*n),
            Token::Dash(n) => "-".repeat(*n),
            Token::Underscore(n) => "_".repeat(*n),
            Token::Url(s) => s.to_string(),
            Token::Whitespace(s) => s.to_string(),
            Token::OpeningBracket => '['.to_string(),
            Token::ClosingBracket => ']'.to_string(),
            Token::OpeningParenthesis => '('.to_string(),
            Token::ClosingParenthesis => '('.to_string(),
            Token::AngleBracket => '>'.to_string(),
            Token::ExclamationMark => '!'.to_string(),
            Token::EOF => String::new(),
        }
    }
}

pub struct Tokenizer<'a> {
    chars: &'a mut CharIterator,
}

impl<'a> Tokenizer<'a> {
    pub fn new(chars: &'a mut CharIterator) -> Self {
        Self { chars }
    }

    pub fn consume(&mut self) -> Token {
        let current = self.chars.current();

        if current == Bytes::Eof {
            return Token::EOF;
        }

        let char = current.char();
        match char {
            '#' | '*' | '`' | '_' | '-' => self.consume_delim(),
            '(' | ')' | '[' | ']' => {
                self.chars.read();
                Token::from_str(&char.to_string()).unwrap()
            }
            _ if char.is_whitespace() => self.consume_whitespace(),
            _ => self.consume_string(),
        }
    }

    pub fn consume_whitespace(&mut self) -> Token {
        let mut whitespace = String::new();
        while self.chars.current().char().is_whitespace() {
            whitespace.push(self.chars.read().char());
        }

        Token::Whitespace(whitespace)
    }

    pub fn consume_string(&mut self) -> Token {
        let mut string = String::new();
        loop {
            let current = self.chars.current();
            let char = current.char();

            if char.is_whitespace()
                || char == '['
                || char == ']'
                || char == '('
                || char == ')'
                || char == '#'
                || char == '*'
                || char == '_'
                || char == '!'
                || current == Bytes::Eof
            {
                break;
            }

            string.push(self.chars.read().char())
        }

        Token::String(string)
    }

    pub fn consume_delim(&mut self) -> Token {
        let mut count = 1;
        let delim = self.chars.read().char();

        while self.chars.current().char() == delim {
            count += 1;
            self.chars.read();
        }

        match delim {
            '#' => Token::Hash(count),
            '*' => Token::Asterisk(count),
            '`' => Token::Backticks(count),
            '-' => Token::Dash(count),
            '_' => Token::Underscore(count),
            // todo: better error handling
            _ => panic!("unexpected delim: {:?}", delim),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bytes::Encoding;

    #[test]
    fn consume_delims() {
        let mut chars = CharIterator::new();
        chars.read_from_str("#####**```---__", Some(Encoding::UTF8));
        let mut tokenizer = Tokenizer::new(&mut chars);

        assert_eq!(tokenizer.consume_delim(), Token::Hash(5));
        assert_eq!(tokenizer.consume_delim(), Token::Asterisk(2));
        assert_eq!(tokenizer.consume_delim(), Token::Backticks(3));
        assert_eq!(tokenizer.consume_delim(), Token::Dash(3));
        assert_eq!(tokenizer.consume_delim(), Token::Underscore(2));
    }

    #[test]
    fn consume_token_stream() {
        let mut chars = CharIterator::new();
        chars.read_from_str(
            "
        ### heading
        **bold**
        _italic_
        [text](link)
        ",
            Some(Encoding::UTF8),
        );
        let mut tokenizer = Tokenizer::new(&mut chars);

        let tokens = vec![
            Token::Whitespace("\n        ".to_string()),
            // ### heading
            Token::Hash(3),
            Token::Whitespace(" ".to_string()),
            Token::String("heading".to_string()),
            Token::Whitespace("\n        ".to_string()),
            // **bold**
            Token::Asterisk(2),
            Token::String("bold".to_string()),
            Token::Asterisk(2),
            Token::Whitespace("\n        ".to_string()),
            // _italic_
            Token::Underscore(1),
            Token::String("italic".to_string()),
            Token::Underscore(1),
            Token::Whitespace("\n        ".to_string()),
            // [text](link)
            Token::OpeningBracket,
            Token::String("text".to_string()),
            Token::ClosingBracket,
            Token::OpeningParenthesis,
            Token::String("link".to_string()),
            Token::ClosingParenthesis,
        ];

        for token in tokens {
            assert_eq!(tokenizer.consume(), token);
        }
    }
}
