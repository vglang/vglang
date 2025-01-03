//! Defines token combinators.

use crate::{Error, Parser, Source, Span};

/// A parser comsume chars until `F` returns *true*.
pub fn take_until<F>(mut f: F) -> impl Parser<Output = Option<Span>, Error = Error>
where
    F: FnMut(char) -> bool + Clone + 'static,
{
    move |source: &mut Source<'_>| take_while(move |c| !f(c)).parse(source)
}

/// A parser comsume chars until `F` returns *true*.
pub fn take_until_enumerate<F>(mut f: F) -> impl Parser<Output = Option<Span>, Error = Error>
where
    F: FnMut(usize, char) -> bool + Clone + 'static,
{
    move |source: &mut Source<'_>| take_while_enumerate(move |index, c| !f(index, c)).parse(source)
}

/// Create [`Until`] parser.
pub fn take_while<F>(mut f: F) -> impl Parser<Output = Option<Span>, Error = Error>
where
    F: FnMut(char) -> bool + Clone + 'static,
{
    take_while_enumerate(move |_, c| f(c))
}

/// Create [`Until`] parser.
pub fn take_while_enumerate<F>(mut f: F) -> impl Parser<Output = Option<Span>, Error = Error>
where
    F: FnMut(usize, char) -> bool + Clone + 'static,
{
    move |source: &mut Source<'_>| {
        let mut index = 0;
        if let Ok((c, start)) = source.next() {
            if !f(index, c) {
                source.seek(start)?;
                return Ok(None);
            }

            index += 1;

            let mut end = start;

            while let Ok((c, span)) = source.next() {
                end = span;

                if !f(index, c) {
                    source.seek(span)?;
                    return Ok(Some(source.extend_to(start, span)));
                }

                index += 1;
            }

            return Ok(Some(source.extend_to_inclusive(start, end)));
        }

        return Ok(None);
    }
}

/// Defines a keyword parser.
pub fn keyword(word: &'static str) -> impl Parser<Output = Span, Error = Error> {
    let mut chars = word.chars();

    take_while(move |c| {
        if let Some(next) = chars.next() {
            next == c
        } else {
            false
        }
    })
    .map_res(move |source, v| {
        if let Some(v) = v {
            if v.len == word.len() {
                return Ok(v);
            }

            source.seek(v)?;
        } else {
        }

        let span = source.span().ok_or(Error::Eof)?;

        return Err(Error::Keyword(word, span));
    })
}

/// Defines a keyword parser.
pub fn is_char(c: char) -> impl Parser<Output = Span, Error = Error> {
    move |source: &mut Source<'_>| {
        let (next, span) = source.next()?;

        if c != next {
            return Err(Error::Char(c, span));
        }

        Ok(span)
    }
}

/// Create a fn that returns true till certain characters are met.
pub fn is_not(characters: &'static str) -> impl Fn(char) -> bool + Clone + 'static {
    move |c| characters.chars().find(|v| *v == c).is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Parser, Source};

    #[test]
    fn test_util() {
        let mut source = Source::from("hello world");

        assert_eq!(
            take_until(|c| c.is_ascii_whitespace())
                .parse(&mut source)
                .map(|span| span.map(|span| source.to_str(span))),
            Ok(Some("hello"))
        );

        let (c, _) = source.next().unwrap();

        assert_eq!(c, ' ');

        assert_eq!(
            take_until(|c| c.is_ascii_whitespace())
                .parse(&mut source)
                .map(|span| span.map(|span| source.to_str(span))),
            Ok(Some("world"))
        );

        assert_eq!(
            take_until(|c| c.is_ascii_whitespace()).parse(&mut source),
            Ok(None)
        );
    }

    #[test]
    fn test_while() {
        let mut source = Source::from("hello world");

        assert_eq!(
            take_while(|c| c.is_ascii_alphanumeric())
                .parse(&mut source)
                .map(|span| span.map(|span| source.to_str(span))),
            Ok(Some("hello"))
        );

        let (c, _) = source.next().unwrap();

        assert_eq!(c, ' ');

        assert_eq!(
            take_while(|c| c.is_ascii_alphanumeric())
                .parse(&mut source)
                .map(|span| span.map(|span| source.to_str(span))),
            Ok(Some("world"))
        );

        use crate::Parser;
    }

    #[test]
    fn test_is_not() {
        let mut source = Source::from("hello \n world");

        assert_eq!(
            take_while(is_not("\n"))
                .parse(&mut source)
                .map(|span| span.map(|span| source.to_str(span))),
            Ok(Some("hello "))
        );
    }

    #[test]
    fn test_keyword() {
        let mut source = Source::from("fntest");

        assert_eq!(
            keyword("fn")
                .parse(&mut source)
                .map(|span| source.to_str(span)),
            Ok("fn")
        );

        assert_eq!(
            keyword("fn").parse(&mut Source::from("test")),
            Err(Error::Keyword(
                "fn",
                Span {
                    lines: 1,
                    cols: 1,
                    offset: 0,
                    len: 1
                }
            ))
        );
    }
}
