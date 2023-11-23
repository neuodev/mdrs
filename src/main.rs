#[allow(dead_code)]
mod bytes;
#[allow(dead_code)]
mod tokenizer;

#[allow(dead_code)]
mod parser;

use bytes::{CharIterator, Encoding};
use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let test_md = include_str!("../TEST.md");

    let mut chars = CharIterator::new();
    chars.read_from_str(test_md, Some(Encoding::UTF8));

    let mut tokenizer = Tokenizer::new(&mut chars);
    let mut parser = Parser::new(&mut tokenizer);

    let doc_ast = parser.parse();

    println!("{:#?}", doc_ast)
}
