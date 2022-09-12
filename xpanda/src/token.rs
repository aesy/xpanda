#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token<'a> {
    Text(&'a str),
    Identifier(&'a str),
    Index(usize),
    OpenBrace,
    CloseBrace,
    DollarSign,
    Colon,
    Dash,
    Plus,
    QuestionMark,
    PoundSign,
}
