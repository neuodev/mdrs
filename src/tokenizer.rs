use crate::bytes::CharIterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    String(String),
    Hash(usize),
    Asterisk(usize),
    Backticks(usize),
    Dash(usize),
    Underscore(usize),
    Url(String),
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
    AngleBracket,
    Whitespace,
    EOF,
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
            _ => todo!(),
        };

        Ok(token)
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
        let char = self.chars.current().char();

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
        while self.chars.current().char().is_whitespace() {
            self.chars.read();
        }

        Token::Whitespace
    }

    pub fn consume_string(&mut self) -> Token {
        let mut string = String::new();
        loop {
            let char = self.chars.current().char();

            if char.is_whitespace()
                || char == '['
                || char == ']'
                || char == '('
                || char == ')'
                || char == '#'
                || char == '*'
                || char == '_'
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
            Token::Whitespace,
            // ### heading
            Token::Hash(3),
            Token::Whitespace,
            Token::String("heading".to_string()),
            Token::Whitespace,
            // **bold**
            Token::Asterisk(2),
            Token::String("bold".to_string()),
            Token::Asterisk(2),
            Token::Whitespace,
            // _italic_
            Token::Underscore(1),
            Token::String("italic".to_string()),
            Token::Underscore(1),
            Token::Whitespace,
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
