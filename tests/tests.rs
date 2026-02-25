#[cfg(test)]
mod tests {
    use lisp_parser::{parser::Parser, scanner::Scanner};

    fn test_parse(input: &str, expected: &str) {
        let scanner = Scanner::new(input);
        let tokens = scanner.scan();
        let mut parser = Parser::new(tokens);
        let result = parser.parse().expect("parse failed").to_string();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty() {
        test_parse("", "[]");
    }

    #[test]
    fn test_parse_number() {
        test_parse("42", "42");
    }

    #[test]
    fn test_parse_identifier() {
        test_parse("foo", "\"foo\"");
    }

    #[test]
    fn test_parse_identifiers() {
        test_parse("(foo bar)", "[\"foo\", \"bar\"]");
    }

    #[test]
    fn test_parse_simple_list() {
        test_parse("(+ 1 2)", "[\"+\", 1, 2]");
    }

    #[test]
    fn test_parse_nested_list() {
        test_parse(
            "(first (list 1 (+ 2 3) 9))",
            "[\"first\", [\"list\", 1, [\"+\", 2, 3], 9]]",
        );
    }
}
