use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "m_expr.pest"]
pub struct MexprParser;

#[derive(Debug, Clone, PartialEq)]
pub enum Mexpr {
    Apply { name: String, body: Vec<Mexpr> },
    List(Vec<Mexpr>),
    Name(String),
    Decimal(String),
    Hexdecimal(String),
    String(String),
}

impl Mexpr {
    pub fn from_parsed(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::mexpr => {
                let list = pair
                    .into_inner()
                    .filter(|r| r.as_rule() != Rule::EOI)
                    .map(Mexpr::from_parsed).collect();
                Mexpr::List(list)
            },
            Rule::m => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_owned();
                let body = inner.map(Mexpr::from_parsed).collect();

                Mexpr::Apply { name, body }
            },
            Rule::list =>
                Mexpr::List(
                    pair.into_inner()
                    .map(Mexpr::from_parsed)
                    .collect()
                ),
            Rule::name =>
                Mexpr::Name(pair.as_str().to_owned()),
            Rule::decimal =>
                Mexpr::Decimal(pair.as_str().to_owned()),
            Rule::hexdecimal =>
                Mexpr::Hexdecimal(pair.as_str().to_owned()),
            Rule::string =>
                Mexpr::String(pair.into_inner().next().unwrap().as_str().to_owned()),
            _ => panic!("{:?}", pair)
        }
    }
}
