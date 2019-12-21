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

impl std::fmt::Display for Mexpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Mexpr::*;

        match self {
            Apply { name, body } => {
                write!(f, "{}[", name)?;
                if !body.is_empty() {
                    body[0].fmt(f)?;

                    for e in &body[1..] {
                        write!(f, ", ")?;
                        e.fmt(f)?;
                    }
                }

                write!(f, "]")?;
            }
            List(body) => {
                write!(f, "[")?;
                if !body.is_empty() {
                    body[0].fmt(f)?;

                    for e in &body[1..] {
                        write!(f, ", ")?;
                        e.fmt(f)?;
                    }
                }

                write!(f, "]")?;
            }
            Name(name) => name.fmt(f)?,
            Decimal(dec) => dec.fmt(f)?,
            Hexdecimal(hex) => hex.fmt(f)?,
            String(s) => write!(f, "\"{}\"", s)?,
        }

        Ok(())
    }
}

impl Mexpr {
    pub fn from_parsed(pair: Pair<Rule>) -> Self {
        match pair.as_rule() {
            Rule::mexpr => {
                let list = pair
                    .into_inner()
                    .filter(|r| r.as_rule() != Rule::EOI)
                    .map(Mexpr::from_parsed)
                    .collect();
                Mexpr::List(list)
            }
            Rule::m => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str().to_owned();
                let body = inner.map(Mexpr::from_parsed).collect();

                Mexpr::Apply { name, body }
            }
            Rule::list => Mexpr::List(pair.into_inner().map(Mexpr::from_parsed).collect()),
            Rule::name => Mexpr::Name(pair.as_str().to_owned()),
            Rule::decimal => Mexpr::Decimal(pair.as_str().to_owned()),
            Rule::hexdecimal => Mexpr::Hexdecimal(pair.as_str().to_owned()),
            Rule::string => Mexpr::String(pair.into_inner().next().unwrap().as_str().to_owned()),
            _ => panic!("{:?}", pair),
        }
    }
}
