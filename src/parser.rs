use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug)]
pub struct Link {
    tokens: Vec<InlineToken>,
    href: String,
}

#[derive(Debug)]
pub enum InlineToken {
    Text(String),
    Link(Link),
    Bold(Vec<InlineToken>),
    Italic(Vec<InlineToken>),
    Code(String),
    Image(String),
}

#[derive(Debug)]
pub struct Paragraph(Vec<InlineToken>);

#[derive(Debug)]
pub struct Heading {
    level: usize,
    tokens: Vec<InlineToken>,
}

#[derive(Debug)]
pub enum ListKind {
    Ordered,
    Unordered,
}

#[derive(Debug)]
pub struct List {
    kind: ListKind,
    items: Vec<ListItem>,
}

pub type ListItem = Vec<Element>;

#[derive(Debug)]
pub struct Document(Vec<Element>);

#[derive(Debug)]
pub enum Element {
    Heading(Heading),
    Paragraph(Paragraph),
    List(List),
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
    pub fn parse(&mut self) -> Document {
        self.lookahead = Some(self.tokenizer.consume());

        Document(self.parse_elements())
    }

    /// ```txt
    /// Elements
    ///     : Element
    ///     | Elements Element -> Element Element Element ...
    ///     ;
    /// ```
    pub fn parse_elements(&mut self) -> Vec<Element> {
        let mut elements = Vec::new();

        if let Some(token) = self.lookahead.clone() {
            while !token.is_eof() {
                elements.push(self.parse_element())
            }
        }

        elements
    }

    /// ```txt
    /// Element
    ///     : Heading
    ///     | Paragraph
    ///     | List
    ///     ;
    /// ```
    pub fn parse_element(&mut self) -> Element {
        if let Some(token) = self.lookahead.clone() {
            if token.is_hash() {
                return Element::Heading(self.parse_heading());
            }
        }

        todo!()
    }

    /// ```txt
    /// Heading
    ///     : <#-token> InlineTokens
    ///     ;
    /// ```
    pub fn parse_heading(&mut self) -> Heading {
        // consuem <#-token>
        let level = self.eat().to_string().len();
        let tokens = self.parse_inline_tokens();

        Heading { level, tokens }
    }

    /// ```txt
    /// List
    ///     : ListItem ...
    ///     ;
    /// ```
    pub fn parse_list(&mut self) -> List {
        let mut items = Vec::new();
        let mut kind = ListKind::Unordered;

        List { kind, items }
    }

    pub fn parse_ordered_list(&mut self) {}

    pub fn parse_unordered_list(&mut self) {}

    /// ```txt
    /// ListItem
    ///     : <dash-token> Elements
    ///     ;
    /// ```
    pub fn parse_list_item(&mut self) -> ListItem {
        // consuem <dash-token>
        self.eat();
        self.parse_elements()
    }

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
            println!("parse_inline_token: {:?}", token);
            return match token {
                Token::ExclamationMark => todo!(),                    // image
                Token::Backticks(1) => todo!(),                       // code
                Token::Asterisk(1) | Token::Underscore(1) => todo!(), // italic
                Token::Asterisk(2) => todo!(),                        // bold
                Token::OpeningBracket => InlineToken::Link(self.parse_link()),
                Token::String(_) | Token::Whitespace => InlineToken::Text(self.parse_text()),
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

    // todo: remove
    pub fn consume_whitespace(&mut self) {
        if let Some(token) = self.lookahead.clone() {
            if token.is_whitespace() {
                self.eat();
            }
        }
    }
}
