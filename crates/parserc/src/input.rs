use std::{iter::Peekable, str::CharIndices};

use crate::{Kind, Report, Source, Span};

/// A seekable source code stream.
pub struct Input<'a, E> {
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
    /// error stack.
    err_stack: Vec<(E, Span)>,
}

impl<'a, E> From<&'a str> for Input<'a, E> {
    fn from(value: &'a str) -> Self {
        Self {
            source: value,
            iter: value.char_indices().peekable(),
            lines: 1,
            cols: 1,
            offset: 0,
            err_stack: vec![],
        }
    }
}

impl<'a, E> Report for Input<'a, E>
where
    E: From<Kind>,
{
    type Error = E;

    fn report_err(&mut self, error: Self::Error, span: Span) {
        self.err_stack.push((error, span));
    }

    fn pop_err(&mut self) -> Option<(Self::Error, Span)> {
        self.err_stack.pop()
    }
}

impl<'a, E> Source<'a> for Input<'a, E> {
    /// Return the [`Span`] of the next char.
    ///
    /// Use [`eof`](Span::eof) fn to check if the eof is reached.
    #[inline]
    fn span(&mut self) -> Span {
        let (_, span) = self.peek();

        span
    }

    /// Convert span into &str.
    fn as_str(&self, span: Span) -> &str {
        &self.source[span.offset..span.offset + span.len]
    }

    /// Seek to the start of the `span`.
    ///
    /// A seek beyond the end of the stream is not allowed, will cause a panic.
    fn seek(&mut self, span: Span) {
        assert!(span.offset <= self.source.len(), "seek: out of range.");
        if self.offset != span.offset {
            self.offset = span.offset;
            self.cols = span.cols;
            self.lines = span.lines;
            self.iter = self.source[span.offset..].char_indices().peekable();
        }
    }

    /// Returns a tuple where the first element is the reading offset, and second element is the total length of the source code.
    fn size_hint(&mut self) -> (usize, usize) {
        (self.offset, self.source.as_bytes().len())
    }

    /// Returns the unparsed length.
    fn remaining(&mut self) -> usize {
        self.source.as_bytes().len() - self.offset
    }

    /// peek up next char in the reading stream.
    fn peek(&mut self) -> (Option<char>, Span) {
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
    fn next(&mut self) -> (Option<char>, Span) {
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

    use super::*;

    #[test]
    fn test_size_hint() {
        assert_eq!(Input::<Kind>::from("hello world").size_hint(), (0, 11));
        assert_eq!(
            Input::<Kind>::from("你好").size_hint(),
            (0, "你好".as_bytes().len())
        );
    }

    #[test]
    fn test_span() {
        assert_eq!(Input::<Kind>::from("hello world").span().len(), 1);
        assert!(Input::<Kind>::from("").span().eof());
        assert_eq!(Input::<Kind>::from("你好").span().len(), '你'.len_utf8());
    }

    #[test]
    fn test_seek() {
        Input::<Kind>::from("hello world").seek(Span {
            offset: 11,
            len: 0,
            lines: 1,
            cols: 12,
        });

        let r = catch_unwind(|| {
            Input::<Kind>::from("hello world").seek(Span {
                offset: 12,
                len: 0,
                lines: 1,
                cols: 12,
            });
        });

        assert!(r.is_err());
    }

    #[test]
    fn test_input() {
        let mut input = Input::<Kind>::from("你好\nh");

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
