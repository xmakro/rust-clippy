use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_ast::ast;
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_session::declare_lint_pass;
use rustc_span::{BytePos, Span};

declare_clippy_lint! {
    /// ### What it does
    /// Checks doc comments for usage of tab characters.
    ///
    /// ### Why is this bad?
    /// The rust style-guide promotes spaces instead of tabs for indentation.
    /// To keep a consistent view on the source, also doc comments should not have tabs.
    /// Also, explaining ascii-diagrams containing tabs can get displayed incorrectly when the
    /// display settings of the author and reader differ.
    ///
    /// ### Example
    /// ```no_run
    /// ///
    /// /// Struct to hold two strings:
    /// /// 	- first		one
    /// /// 	- second	one
    /// pub struct DoubleString {
    ///    ///
    ///    /// 	- First String:
    ///    /// 		- needs to be inside here
    ///    first_string: String,
    ///    ///
    ///    /// 	- Second String:
    ///    /// 		- needs to be inside here
    ///    second_string: String,
    ///}
    /// ```
    ///
    /// Will be converted to:
    /// ```no_run
    /// ///
    /// /// Struct to hold two strings:
    /// ///     - first        one
    /// ///     - second    one
    /// pub struct DoubleString {
    ///    ///
    ///    ///     - First String:
    ///    ///         - needs to be inside here
    ///    first_string: String,
    ///    ///
    ///    ///     - Second String:
    ///    ///         - needs to be inside here
    ///    second_string: String,
    ///}
    /// ```
    #[clippy::version = "1.41.0"]
    pub TABS_IN_DOC_COMMENTS,
    style,
    "using tabs in doc comments is not recommended"
}

declare_lint_pass!(TabsInDocComments => [TABS_IN_DOC_COMMENTS]);

impl TabsInDocComments {
    fn warn_if_tabs_in_doc(cx: &EarlyContext<'_>, attr: &ast::Attribute) {
        if let ast::AttrKind::DocComment(_, comment) = attr.kind {
            let comment = comment.as_str();

            for (lo, hi) in get_chunks_of_tabs(comment) {
                // +3 skips the opening delimiter
                let new_span = Span::new(
                    attr.span.lo() + BytePos(3 + lo),
                    attr.span.lo() + BytePos(3 + hi),
                    attr.span.ctxt(),
                    attr.span.parent(),
                );
                span_lint_and_sugg(
                    cx,
                    TABS_IN_DOC_COMMENTS,
                    new_span,
                    "using tabs in doc comments is not recommended",
                    "consider using four spaces per tab",
                    "    ".repeat((hi - lo) as usize),
                    Applicability::MaybeIncorrect,
                );
            }
        }
    }
}

impl EarlyLintPass for TabsInDocComments {
    fn check_attribute(&mut self, cx: &EarlyContext<'_>, attribute: &ast::Attribute) {
        Self::warn_if_tabs_in_doc(cx, attribute);
    }
}

///
/// scans the string for groups of tabs and returns the start(inclusive) and end positions
/// (exclusive) of all groups
/// e.g. "sd\tasd\t\taa" will be converted to [(2, 3), (6, 8)] as
///       012 3456 7 89
///         ^-^  ^---^
fn get_chunks_of_tabs(the_str: &str) -> Vec<(u32, u32)> {
    let line_length_way_to_long = "doc comment longer than 2^32 chars";
    let mut spans: Vec<(u32, u32)> = vec![];

    // A tab is a single byte in UTF-8 and no other character's encoding contains that byte, so
    // tab groups can be located on the raw bytes and every position found is a char boundary.
    // `str::find` uses a fast single-byte search, and most doc comments contain no tab at all,
    // so they are scanned once without any allocation.
    let bytes = the_str.as_bytes();
    let mut pos = 0;
    while let Some(offset) = the_str[pos..].find('\t') {
        let start = pos + offset;
        let mut end = start + 1;
        while bytes.get(end) == Some(&b'\t') {
            end += 1;
        }
        spans.push((
            u32::try_from(start).expect(line_length_way_to_long),
            u32::try_from(end).expect(line_length_way_to_long),
        ));
        pos = end;
    }

    spans
}

#[cfg(test)]
mod tests_for_get_chunks_of_tabs {
    use super::get_chunks_of_tabs;

    #[test]
    fn test_unicode_han_string() {
        let res = get_chunks_of_tabs(" \u{4f4d}\t");

        assert_eq!(res, vec![(4, 5)]);
    }

    #[test]
    fn test_empty_string() {
        let res = get_chunks_of_tabs("");

        assert_eq!(res, vec![]);
    }

    #[test]
    fn test_simple() {
        let res = get_chunks_of_tabs("sd\t\t\taa");

        assert_eq!(res, vec![(2, 5)]);
    }

    #[test]
    fn test_only_t() {
        let res = get_chunks_of_tabs("\t\t");

        assert_eq!(res, vec![(0, 2)]);
    }

    #[test]
    fn test_only_one_t() {
        let res = get_chunks_of_tabs("\t");

        assert_eq!(res, vec![(0, 1)]);
    }

    #[test]
    fn test_double() {
        let res = get_chunks_of_tabs("sd\tasd\t\taa");

        assert_eq!(res, vec![(2, 3), (6, 8)]);
    }

    #[test]
    fn test_start() {
        let res = get_chunks_of_tabs("\t\taa");

        assert_eq!(res, vec![(0, 2)]);
    }

    #[test]
    fn test_end() {
        let res = get_chunks_of_tabs("aa\t\t");

        assert_eq!(res, vec![(2, 4)]);
    }

    #[test]
    fn test_start_single() {
        let res = get_chunks_of_tabs("\taa");

        assert_eq!(res, vec![(0, 1)]);
    }

    #[test]
    fn test_end_single() {
        let res = get_chunks_of_tabs("aa\t");

        assert_eq!(res, vec![(2, 3)]);
    }

    #[test]
    fn test_no_tabs() {
        let res = get_chunks_of_tabs("dsfs");

        assert_eq!(res, vec![]);
    }
}
