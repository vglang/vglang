//! Defines token combinators.

use crate::{Error, Parser, Source, Span};

/// A parser comsume chars until `F` returns *true*.
pub fn take_until<'a, F>(f: F) -> impl Parser<Output = (Span, Option<Span>), Error = Error>
where
    F: Fn(char) -> bool + 'static,
{
    move |source: &mut Source<'_>| {
        let start = source.span();

        while let Some((c, span)) = source.next() {
            if f(c) {
                source.seek(span)?;
                return Ok((start, Some(span)));
            }
        }

        return Ok((start, None));
    }
}

/// Create [`Until`] parser.
pub fn take_while<'a, F>(f: F) -> impl Parser<Output = (Span, Option<Span>), Error = Error>
where
    F: Fn(char) -> bool + 'static,
{
    move |source: &mut Source<'_>| {
        let start = source.span();

        while let Some((c, span)) = source.next() {
            if !f(c) {
                source.seek(span)?;
                return Ok((start, Some(span)));
            }
        }

        return Ok((start, None));
    }
}

/// Create a fn that returns true till certain characters are met.
pub fn is_not(characters: &'static str) -> impl Fn(char) -> bool + 'static {
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
                .map(|(start, end)| source.as_str(start, end)),
            Ok("hello")
        );

        let (c, _) = source.next().unwrap();

        assert_eq!(c, ' ');

        assert_eq!(
            take_until(|c| c.is_ascii_whitespace())
                .parse(&mut source)
                .map(|(start, end)| source.as_str(start, end)),
            Ok("world")
        );
    }

    #[test]
    fn test_while() {
        let mut source = Source::from("hello world");

        assert_eq!(
            take_while(|c| c.is_ascii_alphanumeric())
                .parse(&mut source)
                .map(|(start, end)| source.as_str(start, end)),
            Ok("hello")
        );

        let (c, _) = source.next().unwrap();

        assert_eq!(c, ' ');

        assert_eq!(
            take_while(|c| c.is_ascii_alphanumeric())
                .parse(&mut source)
                .map(|(start, end)| source.as_str(start, end)),
            Ok("world")
        );

        use crate::Parser;
    }

    #[test]
    fn test_is_not() {
        let mut source = Source::from("hello \n world");

        assert_eq!(
            take_while(is_not("\n"))
                .parse(&mut source)
                .map(|(start, end)| source.as_str(start, end)),
            Ok("hello ")
        );
    }
}
