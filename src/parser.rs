use crate::tokenizer::{Token, Tokenizer};

pub struct Link {
    tokens: Vec<InlineToken>,
    href: String,
}

pub enum InlineToken {
    Text(String),
    Link(Link),
    Bold(Vec<InlineToken>),
    Italic(Vec<InlineToken>),
    Code(String),
    Image(String),
}

pub struct Heading {
    level: usize,
    text: String,
}

pub struct Parser<'stream> {
    tokenizer: &'stream mut Tokenizer<'stream>,
    lookahead: Option<Token>,
}

impl<'stream> Parser<'stream> {
    pub fn new(tokenizer: &'stream mut Tokenizer<'stream>) -> Self {
        Self {
            tokenizer,
            lookahead: None,
        }
    }

    /// ```txt
    /// Document
    ///     : Elements
    ///     ;
    /// ```
    pub fn parse(&mut self) {
        self.lookahead = Some(self.tokenizer.consume());
    }

    /// ```txt
    /// Elements
    ///     : Element
    ///     | Elements Element -> Element Element Element ...
    ///     ;
    /// ```
    pub fn parse_elements(&mut self) {}

    /// ```txt
    /// Element
    ///     : Heading
    ///     | Paragraph
    ///     ;
    /// ```
    pub fn parse_element(&mut self) {}

    /// ```txt
    /// Heading
    ///     : <#-token> InlineTokens
    ///     ;
    /// ```
    pub fn parse_heading(&mut self) {}

    /// ```txt
    /// InlineTokens
    ///     : InlineToken
    ///     | InlineTokens InlineToken -> InlineToken InlineToken InlineToken ...
    ///     ;
    /// ```
    pub fn parse_inline_tokens(&mut self) -> Vec<InlineToken> {
        let mut tokens = Vec::new();

        if let Some(token) = self.lookahead.clone() {
            while !token.is_eof() {
                tokens.push(self.parse_inline_token())
            }
        }

        tokens
    }

    /// ```txt
    /// InlineTokens
    ///     : Text
    ///     | Link
    ///     | Bold
    ///     | Italic
    ///     | Code
    ///     | Image
    ///     ;
    /// ```
    pub fn parse_inline_token(&mut self) -> InlineToken {
        if let Some(token) = self.lookahead.clone() {
            return match token {
                Token::ExclamationMark => todo!(),                    // image
                Token::Backticks(1) => todo!(),                       // code
                Token::Asterisk(1) | Token::Underscore(1) => todo!(), // italic
                Token::Asterisk(2) => todo!(),                        // bold
                Token::OpeningBracket => InlineToken::Link(self.parse_link()),
                Token::String(_) => InlineToken::Text(self.parse_text()),
                _ => todo!(),
            };
        }

        todo!()
    }

    /// ```txt
    /// Text
    ///   : <string-token> ...
    ///   ;
    /// ```
    pub fn parse_text(&mut self) -> String {
        let mut text = String::new();

        loop {
            if let Some(token) = self.lookahead.clone() {
                if token.is_whitespace() {
                    self.eat();
                }

                if token.is_string() {
                    text.push_str(&self.eat().to_string())
                }
            }

            break;
        }

        text
    }

    /// ```txt
    /// Link
    ///   : <[-token> InlineTokens <]-token> <(-token> Text  <)-token>
    ///   ;
    /// ```
    pub fn parse_link(&mut self) -> Link {
        // todo: error handling

        // consume <[-token>
        self.tokenizer.consume();

        let tokens = self.parse_inline_tokens();

        // consume <]-token>
        self.tokenizer.consume();

        // consume <(-token>
        self.tokenizer.consume();

        let href = self.parse_text();

        // consume <)-token>
        self.tokenizer.consume();

        Link { tokens, href }
    }

    pub fn eat(&mut self) -> Token {
        if let Some(token) = self.lookahead.clone() {
            self.lookahead = Some(self.tokenizer.consume());
            return token;
        }

        todo!()
    }
}
