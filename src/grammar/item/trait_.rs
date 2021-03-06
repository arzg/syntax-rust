use crate::grammar::parse_item;
use crate::Parser;
use dialect::HighlightGroup;

pub(super) fn parse_trait(p: &mut Parser) {
    assert!(p.at(&[crate::TokenKind::Trait]));
    p.eat(HighlightGroup::OtherKeyword);

    p.push(crate::TokenKind::TypeIdent, HighlightGroup::InterfaceDef);

    p.push(crate::TokenKind::OpenBrace, HighlightGroup::Delimiter);

    loop {
        if p.at_end() {
            break;
        }

        if p.at(&[crate::TokenKind::CloseBrace]) {
            p.eat(HighlightGroup::Delimiter);
            break;
        }

        parse_item(p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dialect::HighlightedSpan;
    use pretty_assertions::assert_eq;

    #[test]
    fn parses_trait_with_no_items() {
        let mut parser = Parser::new("trait A {}");
        parse_trait(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..5,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::InterfaceDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 9..10,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_trait_with_several_items() {
        let mut parser = Parser::new("trait A { fn a(); }");
        parse_trait(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..5,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::InterfaceDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 10..12,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 13..14,
                    group: HighlightGroup::FunctionDef,
                },
                HighlightedSpan {
                    range: 14..15,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 15..16,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 16..17,
                    group: HighlightGroup::Terminator,
                },
                HighlightedSpan {
                    range: 18..19,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_trait_with_erroneous_contents() {
        let mut parser = Parser::new("trait A { not valid }");
        parse_trait(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..5,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::InterfaceDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::Delimiter,
                },
                HighlightedSpan {
                    range: 10..13,
                    group: HighlightGroup::Error,
                },
                HighlightedSpan {
                    range: 14..19,
                    group: HighlightGroup::Error,
                },
                HighlightedSpan {
                    range: 20..21,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }

    #[test]
    fn parses_trait_with_unclosed_brace() {
        let mut parser = Parser::new("trait A {");
        parse_trait(&mut parser);

        assert_eq!(
            parser.output,
            vec![
                HighlightedSpan {
                    range: 0..5,
                    group: HighlightGroup::OtherKeyword,
                },
                HighlightedSpan {
                    range: 6..7,
                    group: HighlightGroup::InterfaceDef,
                },
                HighlightedSpan {
                    range: 8..9,
                    group: HighlightGroup::Delimiter,
                },
            ],
        );
    }
}
