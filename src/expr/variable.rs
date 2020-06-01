use {
    crate::{utils::snake_case, ParseResult},
    nom::combinator::map,
};

pub(super) fn parse(s: &str) -> ParseResult<'_> {
    map(snake_case, |s| {
        vec![syntax::HighlightedSpan {
            text: s,
            group: Some(syntax::HighlightGroup::VariableUse),
        }]
    })(s)
}
