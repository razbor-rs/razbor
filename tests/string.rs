use pest::{parses_to, consumes_to};
use razbor::{MexprParser, Rule};

#[test]
fn string() {
    parses_to! {
        parser: MexprParser,
        input:  "\"asd\"",
        rule:   Rule::string,
        tokens: [
            string(0, 5, [
                inner(1, 4)
            ])
        ]
    };
    parses_to! {
        parser: MexprParser,
        input:  "\"\\\"a\"",
        rule:   Rule::string,
        tokens: [
            string(0, 5, [
                inner(1, 4)
            ])
        ]
    };
}
