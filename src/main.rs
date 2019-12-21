use pest::Parser;
use razbor::MexprParser;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let parsed = MexprParser::parse(
        razbor::Rule::mexpr,
        "foo[a, b, \"c d\", 0, 02, 12, 0x01, f[g, []]]",
    )?
    .next()
    .unwrap();
    let expr = razbor::Mexpr::from_parsed(parsed);
    println!("{:?}", expr);

    Ok(())
}
