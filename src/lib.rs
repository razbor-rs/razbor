use pest::iterators::Pair;
use pest_derive::Parser;

use std::path::Path;

pub mod import;
pub mod path;

#[derive(Debug, Clone, PartialEq)]
struct UnknownError;

impl std::fmt::Display for UnknownError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "Unknown error")
    }
}

impl std::error::Error for UnknownError { }

#[derive(Debug, Clone, PartialEq)]
struct StringError(String);

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "ERROR: {}", self.0)
    }
}

impl std::error::Error for StringError { }

// Some day there will be a proper type here
type Error = Box<dyn std::error::Error>;

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

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Mexpr, Error> {
    use pest::Parser;

    let data = std::fs::read_to_string(path).unwrap();
    let parsed =
        MexprParser::parse(Rule::mexpr, &data)?
        .next().ok_or(UnknownError)?;

    Ok(Mexpr::from_parsed(parsed))
}

fn is_m_name(expr: &Mexpr, name: &str) -> bool {
    let n = name;

    match expr {
        Mexpr::Apply { name, .. } =>
            n == name,
        _ => false,
    }
}

fn destruct_apply(expr: Mexpr) -> Option<(String, Vec<Mexpr>)> {
    match expr {
        Mexpr::Apply { name, body } => Some((name, body)),
        _ => None
    }
}


fn destruct_name(expr: Mexpr) -> Option<String> {
    match expr {
        Mexpr::Name(n) => Some(n),
        _ => None
    }
}

fn get_name_value(expr: &[Mexpr]) -> Option<(&String, &Mexpr)> {
    let mut iter = expr.iter();
    let head = iter.next()
        .and_then(|h|
            match h {
                Mexpr::Name(n) => Some(n),
                _ => None,
            }
        )?;
    let tail = iter.next()?;

    if iter.next().is_some() { return None };

    Some((head, tail))
}
