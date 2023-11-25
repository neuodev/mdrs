use crate::bytes::{CharIterator, Encoding};
use crate::tokenizer::{Token, Tokenizer};

#[derive(Debug, PartialEq, Eq)]
pub struct Link {
    tokens: Vec<InlineToken>,
    href: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Image {
    src: String,
    alt: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InlineToken {
    Text(String),
    Link(Link),
    Image(Image),
    Bold(Vec<InlineToken>),
    Italic(Vec<InlineToken>),
    Code(String),
}

impl InlineToken {
    pub fn new_text(text: &str) -> Self {
        InlineToken::Text(text.to_string())
    }

    pub fn new_link(tokens: Vec<InlineToken>, href: &str) -> Self {
        InlineToken::Link(Link {
            tokens,
            href: href.to_string(),
        })
    }

    pub fn new_blod(tokens: Vec<InlineToken>) -> Self {
        InlineToken::Bold(tokens)
    }

    pub fn new_italic(tokens: Vec<InlineToken>) -> Self {
        InlineToken::Italic(tokens)
    }

    pub fn new_code(code: &str) -> Self {
        InlineToken::Code(code.to_string())
    }

    pub fn new_img(src: &str, alt: &str) -> Self {
        InlineToken::Image(Image {
            src: src.to_string(),
            alt: alt.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Paragraph(Vec<InlineToken>);

#[derive(Debug, PartialEq, Eq)]
pub struct Heading {
    level: usize,
    tokens: Vec<InlineToken>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListKind {
    Ordered,
    Unordered,
}

#[derive(Debug, PartialEq, Eq)]
pub struct List {
    kind: ListKind,
    items: Vec<ListItem>,
}

pub type ListItem = Vec<Element>;

#[derive(Debug, PartialEq, Eq)]
pub struct Document(Vec<Element>);

impl Document {
    pub fn new(elements: Vec<Element>) -> Self {
        Self(elements)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    Heading(Heading),
    Paragraph(Paragraph),
    List(List),
}

impl Element {
    pub fn new_heading(level: usize, tokens: Vec<InlineToken>) -> Self {
        Element::Heading(Heading { level, tokens })
    }

    pub fn new_paragraph(tokens: Vec<InlineToken>) -> Self {
        Element::Paragraph(Paragraph(tokens))
    }

    pub fn new_list(kind: ListKind, items: Vec<ListItem>) -> Self {
        Element::List(List { kind, items })
    }
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

        loop {
            println!("parse_elements loops");
            if let Some(token) = self.lookahead.clone() {
                if !token.is_eof() {
                    elements.push(self.parse_element())
                } else {
                    break;
                }
            } else {
                break;
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

        loop {
            println!("parse_inline_tokens loops");
            if let Some(token) = self.lookahead.clone() {
                if !token.is_eof() {
                    tokens.push(self.parse_inline_token())
                } else {
                    break;
                }
            } else {
                break;
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
                Token::String(_) | Token::Whitespace(_) => InlineToken::Text(self.parse_text()),
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
            println!("parse_text");
            if let Some(token) = self.lookahead.clone() {
                if token.is_whitespace() {
                    text.push_str(&self.eat().to_string());
                    continue;
                }

                if token.is_string() {
                    text.push_str(&self.eat().to_string());
                    continue;
                }

                if token.is_eof() {
                    break;
                }

                break;
            } else {
                break;
            }
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

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_ast {
        ($raw:expr, $doc_ast:expr) => {
            let mut chars = CharIterator::new();
            chars.read_from_str($raw, Some(Encoding::UTF8));

            let mut tokenizer = Tokenizer::new(&mut chars);
            let mut parser = Parser::new(&mut tokenizer);

            assert_eq!(parser.parse(), $doc_ast);
        };
    }

    #[test]
    fn parse_heading() {
        let tests = vec![
            ("# h1", 1, " h1"),
            ("## h2", 2, " h2"),
            ("### h3", 3, " h3"),
            ("#### I am heading", 4, " I am heading"),
        ];
        for (raw, level, text) in tests {
            assert_ast!(
                raw,
                Document::new(vec![Element::new_heading(
                    level,
                    vec![InlineToken::new_text(text)]
                )])
            );
        }
    }
}
