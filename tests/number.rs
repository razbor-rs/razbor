use pest::{consumes_to, parses_to, Parser};
use razbor::{MexprParser, Rule};

#[test]
fn decimal() {
    parses_to! {
        parser: MexprParser,
        input:  "0",
        rule:   Rule::number,
        tokens: [
            decimal(0, 1)
        ]
    };

    parses_to! {
        parser: MexprParser,
        input:  "42",
        rule:   Rule::number,
        tokens: [
            decimal(0, 2)
        ]
    };
}

#[test]
fn hexadecimal() {
    parses_to! {
        parser: MexprParser,
        input:  "0x42",
        rule:   Rule::hexdecimal,
        tokens: [
            hexdecimal(0, 4)
        ]
    };

    let parsed = MexprParser::parse(razbor::Rule::hexdecimal, "0x");
    assert!(parsed.is_err());
}

#[test]
fn number() {
    parses_to! {
        parser: MexprParser,
        input:  "[0, 1, 12, 007, 0x42]",
        rule:   Rule::list,
        tokens: [
            list(0, 21, [
                decimal(1, 2),
                decimal(4, 5),
                decimal(7, 9),
                decimal(11, 14),
                hexdecimal(16, 20),
            ])
        ]
    };
}
