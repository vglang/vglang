use std::{
    fmt::{Debug, Display},
    iter::Peekable,
    str::CharIndices,
    vec::IntoIter,
};

/// A `span` is a reference to a fragment of the source code.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Span {
    pub offset: usize,
    pub len: usize,
    pub lines: usize,
    pub cols: usize,
}

impl From<(usize, usize, usize, usize)> for Span {
    fn from(value: (usize, usize, usize, usize)) -> Self {
        Self {
            offset: value.0,
            len: value.1,
            lines: value.2,
            cols: value.3,
        }
    }
}

impl Span {
    /// Create a new `Span` instance.
    ///
    /// A length of `0` indicates that the span reference to the `eof`
    pub fn new(offset: usize, len: usize, lines: usize, cols: usize) -> Self {
        Self {
            offset,
            len,
            lines,
            cols,
        }
    }

    /// Returns true if the `len` is `0`.
    pub fn eof(&self) -> bool {
        self.len == 0
    }

    /// Returns the postion`(lines,cols)` referenced by this span.
    pub fn position(&self) -> (usize, usize) {
        (self.lines, self.cols)
    }

    /// Return the length of this span in bytes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Extend self to `other`'s start offset.
    pub fn extend_to(self, other: Span) -> Span {
        assert!(
            !(self.offset > other.offset),
            "extend_to: self.offset < other.offset."
        );

        Span {
            offset: self.offset,
            len: other.offset - self.offset,
            lines: self.lines,
            cols: self.cols,
        }
    }

    /// Extend self to `other`'s end offset.
    pub fn extend_to_inclusive(self, other: Span) -> Span {
        assert!(
            !(self.offset > other.offset),
            "extend_to: self.offset < other.offset."
        );

        Span {
            offset: self.offset,
            len: other.offset + other.len - self.offset,
            lines: self.lines,
            cols: self.cols,
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Ln {}, Col {}]", self.lines, self.cols)
    }
}
/// Report for parsing error.
enum ReportRecord {
    /// Error report start tag.
    Start,
    /// Error report end tag.
    End,
    /// This is the origin error report.
    #[allow(unused)]
    Err(anyhow::Error, Span),
}

impl Debug for ReportRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportRecord::Start => write!(f, "start"),
            ReportRecord::End => write!(f, "end"),
            ReportRecord::Err(_, span) => {
                write!(f, "err({:?})", span)
            }
        }
    }
}

impl PartialEq for ReportRecord {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ReportRecord::Start => {
                if let ReportRecord::Start = other {
                    true
                } else {
                    false
                }
            }
            ReportRecord::End => {
                if let ReportRecord::End = other {
                    true
                } else {
                    false
                }
            }
            ReportRecord::Err(_, span) => {
                if let ReportRecord::Err(_, other_span) = other {
                    *span == *other_span
                } else {
                    false
                }
            }
        }
    }
}

impl<E, S> From<(E, S)> for ReportRecord
where
    Span: From<S>,
    anyhow::Error: From<E>,
{
    fn from(value: (E, S)) -> Self {
        Self::Err(value.0.into(), value.1.into())
    }
}

/// A error type returned by [ParseContext::report]
pub struct ReportLine(pub(crate) anyhow::Error, pub Span);

impl ReportLine {
    /// Returns the position of this error was been reported.
    pub fn span(&self) -> Span {
        self.1
    }

    pub fn downcast_ref<E>(&self) -> Option<&E>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        self.0.downcast_ref()
    }

    pub fn downcast<E>(self) -> Result<(E, Span), Self>
    where
        E: Display + Debug + Send + Sync + 'static,
    {
        match self.0.downcast::<E>() {
            Ok(err) => return Ok((err, self.1)),
            Err(err) => return Err(ReportLine(err, self.1)),
        }
    }
}

impl<E, S> From<(E, S)> for ReportLine
where
    Span: From<S>,
    anyhow::Error: From<E>,
{
    fn from(value: (E, S)) -> Self {
        Self(value.0.into(), value.1.into())
    }
}

/// Iterator returns by [`ParseContext::report`] fn.
pub struct ReportIter(IntoIter<ReportRecord>);

impl ReportIter {
    /// print error report into console.
    pub fn eprint(self) {
        for (index, report) in self.enumerate() {
            let report = report
                .into_iter()
                .enumerate()
                .map(|(index, line)| format!("   {}: {} {}", index, line.0, line.span()))
                .collect::<Vec<_>>()
                .join("\n");

            println!("parserc error({})\n{}", index, report);
        }
    }
}

impl Iterator for ReportIter {
    type Item = Vec<ReportLine>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut records = vec![];

        while let Some(record) = self.0.next() {
            match record {
                ReportRecord::Start => {}
                ReportRecord::End => return Some(records),
                ReportRecord::Err(error, span) => {
                    records.push(ReportLine(error, span));
                }
            }
        }

        assert!(records.is_empty(), "inner error");

        return None;
    }
}
/// A seekable source code stream.
pub struct ParseContext<'a> {
    /// raw source code.
    source: &'a str,
    /// the char stream iterator.
    iter: Peekable<CharIndices<'a>>,
    /// current reading offset. start with '0'
    offset: usize,
    /// tracking the line no. start with '1'
    lines: usize,
    /// tracking the col no. start with `1`
    cols: usize,
    /// error reports.
    error_reports: Vec<ReportRecord>,
}

impl<'a> From<&'a str> for ParseContext<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            source: value,
            iter: value.char_indices().peekable(),
            lines: 1,
            cols: 1,
            offset: 0,
            error_reports: vec![],
        }
    }
}

impl<'a> ParseContext<'a> {
    /// Report a new error.
    #[inline(always)]
    pub fn report_error<E, S>(&mut self, error: E, span: S)
    where
        anyhow::Error: From<E>,
        Span: From<S>,
    {
        self.error_reports.push(ReportRecord::Start);
        self.error_reports
            .push(ReportRecord::Err(error.into(), span.into()));
        self.error_reports.push(ReportRecord::End);
    }

    /// if exists, add context information for last error report, otherwise report a new error.
    #[inline(always)]
    pub fn with_context<E, S>(&mut self, error: E, span: S)
    where
        anyhow::Error: From<E>,
        Span: From<S>,
    {
        if let Some(ReportRecord::End) = self.error_reports.pop() {
            let mut end = None;
            for (index, report) in self.error_reports.iter().rev().enumerate() {
                if let &ReportRecord::Start = report {
                    end = Some(index);
                    break;
                }
            }

            let mut err_stack = self
                .error_reports
                .split_off(self.error_reports.len() - end.unwrap());

            self.error_reports
                .push(ReportRecord::Err(error.into(), span.into()));

            self.error_reports.append(&mut err_stack);

            self.error_reports.push(ReportRecord::End);
        } else {
            self.report_error(error, span);
        }
    }

    /// Returns the report record size.
    #[inline]
    pub fn report_size(&self) -> usize {
        self.error_reports.len()
    }

    /// Returns a clone of report list.
    #[inline]
    pub fn report(&mut self) -> ReportIter {
        ReportIter(self.error_reports.drain(..).collect::<Vec<_>>().into_iter())
    }

    /// Pop up the last report.
    #[inline]
    pub fn last_error(&mut self) -> Option<Vec<ReportLine>> {
        if let Some(ReportRecord::End) = self.error_reports.pop() {
            let mut lines = vec![];

            while let Some(ReportRecord::Err(err, span)) = self.error_reports.pop() {
                lines.push(ReportLine(err, span));
            }

            lines.reverse();

            return Some(lines);
        }

        return None;
    }

    /// Return the [`Span`] of the next char.
    ///
    /// Use [`eof`](Span::eof) fn to check if the eof is reached.
    #[inline]
    pub fn span(&mut self) -> Span {
        let (_, span) = self.peek();

        span
    }

    /// Convert span into &str.
    #[inline(always)]
    pub fn as_str(&self, span: Span) -> &str {
        &self.source[span.offset..span.offset + span.len]
    }

    /// Seek to the start of the `span`.
    ///
    /// A seek beyond the end of the stream is not allowed, will cause a panic.
    #[inline(always)]
    pub fn seek<S>(&mut self, span: S)
    where
        Span: From<S>,
    {
        let span: Span = span.into();

        assert!(span.offset <= self.source.len(), "seek: out of range.");
        if self.offset != span.offset {
            self.offset = span.offset;
            self.cols = span.cols;
            self.lines = span.lines;
            self.iter = self.source[span.offset..].char_indices().peekable();
        }

        // Clearup reporst whose span offset is greater than `seek` position.

        let mut split_to = None;

        for (index, report) in self.error_reports.iter().rev().enumerate() {
            match report {
                ReportRecord::Start => {}
                ReportRecord::End => {}
                ReportRecord::Err(_, span) => {
                    if span.offset < self.offset {
                        break;
                    }

                    split_to = Some(index);
                }
            }
        }

        if let Some(split_to) = split_to {
            // reverse index.
            let split_to = self.error_reports.len() - split_to - 1;

            _ = self.error_reports.split_off(split_to);

            if let Some(&ReportRecord::Start) = self.error_reports.last() {
                self.error_reports.pop();
            }

            match self.error_reports.last() {
                Some(ReportRecord::End) | None => {}
                _ => {
                    self.error_reports.push(ReportRecord::End);
                }
            }
        }
    }

    /// Returns a tuple where the first element is the reading offset, and second element is the total length of the source code.
    #[inline]
    pub fn size_hint(&mut self) -> (usize, usize) {
        (self.offset, self.source.as_bytes().len())
    }

    /// Returns the unparsed length.
    #[inline]
    pub fn remaining(&mut self) -> usize {
        self.source.as_bytes().len() - self.offset
    }

    /// peek up next char in the reading stream.
    #[inline(always)]
    pub fn peek(&mut self) -> (Option<char>, Span) {
        if let Some((_, c)) = self.iter.peek() {
            (
                Some(*c),
                Span {
                    offset: self.offset,
                    len: c.len_utf8(),
                    lines: self.lines,
                    cols: self.cols,
                },
            )
        } else {
            (
                None,
                Span {
                    offset: self.offset,
                    len: 0,
                    lines: self.lines,
                    cols: self.cols,
                },
            )
        }
    }

    /// Returns the next character and its corresponding [`Span`].
    #[inline(always)]
    pub fn next(&mut self) -> (Option<char>, Span) {
        if let Some((_, c)) = self.iter.next() {
            let span = Span {
                offset: self.offset,
                len: c.len_utf8(),
                lines: self.lines,
                cols: self.cols,
            };

            // update tracking datas.
            self.offset += c.len_utf8();
            // is newline.
            if c == '\n' {
                self.lines += 1;
                self.cols = 1;
            } else {
                self.cols += 1;
            }

            (Some(c), span)
        } else {
            (
                None,
                Span {
                    offset: self.offset,
                    len: 0,
                    lines: self.lines,
                    cols: self.cols,
                },
            )
        }
    }
}

#[cfg(test)]
mod tests {

    use std::panic::catch_unwind;

    use anyhow::anyhow;

    use crate::{ensure_char, Parser, ParserExt};

    use super::*;

    impl Debug for ReportLine {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "report({:#?})", self.1)
        }
    }

    impl PartialEq for ReportLine {
        fn eq(&self, other: &Self) -> bool {
            self.1 == other.1
        }
    }

    #[test]
    fn test_size_hint() {
        assert_eq!(ParseContext::from("hello world").size_hint(), (0, 11));
        assert_eq!(
            ParseContext::from("你好").size_hint(),
            (0, "你好".as_bytes().len())
        );
    }

    #[test]
    fn test_span() {
        assert_eq!(ParseContext::from("hello world").span().len(), 1);
        assert!(ParseContext::from("").span().eof());
        assert_eq!(ParseContext::from("你好").span().len(), '你'.len_utf8());
    }

    #[test]
    fn test_seek() {
        ParseContext::from("hello world").seek(Span {
            offset: 11,
            len: 0,
            lines: 1,
            cols: 12,
        });

        let r = catch_unwind(|| {
            ParseContext::from("hello world").seek(Span {
                offset: 12,
                len: 0,
                lines: 1,
                cols: 12,
            });
        });

        assert!(r.is_err());
    }

    #[test]
    fn test_error_report2() {
        let mut input = ParseContext::from("a");
        ensure_char('{').ok().parse(&mut input).unwrap();

        assert_eq!(input.report().count(), 0);

        let mut input = ParseContext::from("");
        ensure_char('{').ok().parse(&mut input).unwrap();

        assert_eq!(input.report().count(), 0);
    }

    #[test]
    fn test_error_report() {
        let mut context = ParseContext::from("hello world");

        context.report_error(anyhow!("one"), (3, 1, 1, 4));

        context.report_error(anyhow!("two"), (5, 1, 1, 6));

        context.with_context(anyhow!("three"), (4, 1, 1, 5));

        assert_eq!(
            context.error_reports,
            &[
                ReportRecord::Start,
                (anyhow!("one"), (3, 1, 1, 4)).into(),
                ReportRecord::End,
                ReportRecord::Start,
                (anyhow!("three"), (4, 1, 1, 5)).into(),
                (anyhow!("two"), (5, 1, 1, 6)).into(),
                ReportRecord::End,
            ]
        );

        context.seek((5, 1, 1, 5));

        assert_eq!(
            context.error_reports,
            &[
                ReportRecord::Start,
                (anyhow!("one"), (3, 1, 1, 4)).into(),
                ReportRecord::End,
                ReportRecord::Start,
                (anyhow!("three"), (4, 1, 1, 5)).into(),
                ReportRecord::End,
            ]
        );

        context.seek((0, 1, 1, 1));

        assert_eq!(context.error_reports, &[]);

        let mut context = ParseContext::from("hello world");

        context.report_error(anyhow!("one"), (3, 1, 1, 4));

        context.report_error(anyhow!("two"), (5, 1, 1, 6));

        context.with_context(anyhow!("three"), (4, 1, 1, 5));

        assert_eq!(context.report_size(), 7);

        let mut report_iter = context.report();

        assert_eq!(
            report_iter.next().unwrap(),
            vec![(anyhow!("one"), (3, 1, 1, 4)).into()]
        );

        assert_eq!(
            report_iter.next().unwrap(),
            vec![
                (anyhow!("three"), (4, 1, 1, 5)).into(),
                (anyhow!("two"), (5, 1, 1, 6)).into()
            ]
        );
    }

    #[test]
    fn test_input() {
        let mut input = ParseContext::from("你好\nh");

        assert_eq!(
            input.next(),
            (
                Some('你'),
                Span {
                    offset: 0,
                    len: 3,
                    lines: 1,
                    cols: 1
                }
            )
        );

        assert_eq!(input.next().0, Some('好'));
        assert_eq!(input.next().0, Some('\n'));
        assert_eq!(
            input.next(),
            (
                Some('h'),
                Span {
                    offset: 7,
                    len: 1,
                    lines: 2,
                    cols: 1
                }
            )
        );

        for _ in 0..10 {
            assert_eq!(
                input.next(),
                (
                    None,
                    Span {
                        offset: 8,
                        len: 0,
                        lines: 2,
                        cols: 2
                    }
                )
            );
        }
    }
}
