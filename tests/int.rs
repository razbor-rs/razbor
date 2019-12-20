use pest::{Parser, parses_to, consumes_to};
use razbor::{MexprParser, Rule};

#[test]
fn decimal() {
    parses_to! {
        parser: MexprParser,
        input:  "0",
        rule:   Rule::int,
        tokens: [
            number(0, 1)
        ]
    };

    parses_to! {
        parser: MexprParser,
        input:  "42",
        rule:   Rule::int,
        tokens: [
            number(0, 2)
        ]
    };
}

#[test]
fn octal() {
    parses_to! {
        parser: MexprParser,
        input:  "00",
        rule:   Rule::int,
        tokens: [
            oct_number(0, 2)
        ]
    };

    parses_to! {
        parser: MexprParser,
        input:  "042",
        rule:   Rule::int,
        tokens: [
            oct_number(0, 3)
        ]
    };
}

#[test]
fn hexadecimal() {
    parses_to! {
        parser: MexprParser,
        input:  "0x42",
        rule:   Rule::int,
        tokens: [
            hex_number(0, 4)
        ]
    };

    let parsed = MexprParser::parse(razbor::Rule::hex_number, "0x");
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
                number(1, 2),
                number(4, 5),
                oct_number(7, 9),
                number(11, 13),
                hex_number(15, 19),
            ])
        ]
    };
}
