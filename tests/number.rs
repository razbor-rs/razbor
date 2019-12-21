use pest::{Parser, parses_to, consumes_to};
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
fn octal() {
    parses_to! {
        parser: MexprParser,
        input:  "00",
        rule:   Rule::number,
        tokens: [
            octal(0, 2)
        ]
    };

    parses_to! {
        parser: MexprParser,
        input:  "042",
        rule:   Rule::number,
        tokens: [
            octal(0, 3)
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
        input:  "[0, 1, 01, 12, 0x42]",
        rule:   Rule::list,
        tokens: [
            list(0, 20, [
                decimal(1, 2),
                decimal(4, 5),
                octal(7, 9),
                decimal(11, 13),
                hexdecimal(15, 19),
            ])
        ]
    };
}
