use std::{marker::PhantomData, str::Chars};

use crate::{Input, Kind, ParserError, Span, ToDiagnostic};

/// A parser produce output by parsing and consuming the [`Input`] char stream.
pub trait Parser {
    /// Output data type.
    type Output;

    /// Error type that this parser may return.
    type Error: ParserError;

    /// Parse and generate a new output.
    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error>;
}

/// Implement [`Parser`] for all [`FnMut`](&mut Input<'_>) -> Result<O, E>.
impl<F, O, E> Parser for F
where
    F: FnOnce(&mut Input<'_>) -> Result<O, E>,
    E: ParserError,
{
    type Output = O;
    type Error = E;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        (self)(input)
    }
}

/// A combinator for [`ok`](ParserExt::ok) function.
#[derive(Clone)]
pub struct Optional<S>(S);

impl<S> Parser for Optional<S>
where
    S: Parser,
{
    type Error = S::Error;
    type Output = Option<S::Output>;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        match self.0.parse(input) {
            Err(err) => match err.diagnostic() {
                crate::Diagnostic::Fatal(_) => Err(err),
                crate::Diagnostic::Incomplete(span) | crate::Diagnostic::Recoverable(span) => {
                    input.seek(*span);
                    Ok(None)
                }
            },
            Ok(output) => Ok(Some(output)),
        }
    }
}

/// A combinator for [`or`](ParserExt::or) function.
#[derive(Clone)]
pub struct Or<S, O>(S, O);

impl<S, O, Output, Error> Parser for Or<S, O>
where
    S: Parser<Output = Output, Error = Error> + Clone,
    O: Parser<Output = Output, Error = Error>,
    Error: ParserError,
{
    type Output = Output;
    type Error = Error;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        if let Some(output) = self.0.clone().ok().parse(input)? {
            return Ok(output);
        } else {
            self.1.parse(input)
        }
    }
}

/// A combinator for [`map`](ParserExt::map) function.
#[derive(Clone)]
pub struct Map<S, F>(S, F);

impl<S, F, U> Parser for Map<S, F>
where
    S: Parser,
    F: FnOnce(S::Output) -> U,
{
    type Output = U;
    type Error = S::Error;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        self.0.parse(input).map(self.1)
    }
}

/// A combinator for [`map_err`](ParserExt::map_err) function.
#[derive(Clone)]
pub struct MapErr<S, F>(S, F);

impl<S, F, U> Parser for MapErr<S, F>
where
    S: Parser,
    F: FnOnce(S::Error) -> U,
    U: ParserError,
{
    type Output = S::Output;
    type Error = U;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        self.0.parse(input).map_err(self.1)
    }
}

/// An extension trait for [`Parser`] combinators.
pub trait ParserExt: Parser {
    /// Convert parser result from [`Recoverable`] / [`Incomplete`] errors to [`None`].
    ///
    /// [`Recoverable`]: crate::Diagnostic::Recoverable
    /// [`Incomplete`]: crate::Diagnostic::Incomplete
    #[inline]
    fn ok(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional(self)
    }

    /// Sequentially execute two parsers, until one of them returns successfully.
    #[inline]
    fn or<O>(self, other: O) -> Or<Self, O>
    where
        Self: Sized + Clone,
    {
        Or(self, other)
    }

    /// Map parser `Result<T,E>` to `Result<U,E>` by applying a function to a contained Ok value, leaving an Err value untouched.
    fn map<F, U>(self, op: F) -> Map<Self, F>
    where
        F: FnOnce(Self::Output) -> U,
        Self: Sized,
    {
        Map(self, op)
    }

    /// Maps a Result<T, E> to Result<T, F> by applying a function to a contained Err value, leaving an Ok value untouched.
    ///
    /// This function can be used to pass through a successful result while handling an error.
    fn map_err<F, U>(self, op: F) -> MapErr<Self, F>
    where
        F: FnOnce(Self::Error) -> U,
        Self: Sized,
    {
        MapErr(self, op)
    }
}

impl<T> ParserExt for T where T: Parser {}

/// All types that can be parsed from the [`Input`] stream must implement this trait.
///
/// See [`parse`](InputExt::parse) function.
pub trait FromInput {
    type Error: ParserError;

    /// Parse and create `Self` from [`Input`] stream.
    fn parse(input: &mut Input<'_>) -> Result<Self, Self::Error>
    where
        Self: Sized;
}

/// A helper trait that convert [`FromInput`] into a [`Parser`].
pub trait IntoParser: FromInput {
    /// Conver self into parser.
    fn into_parser() -> ParserFromInput<Self>
    where
        Self: Sized,
    {
        ParserFromInput(Default::default())
    }
}

impl<T> IntoParser for T where T: FromInput {}

/// A wrapper parser for [`FromInput`] type.
pub struct ParserFromInput<T>(PhantomData<T>);

impl<T> Clone for ParserFromInput<T> {
    fn clone(&self) -> Self {
        Self(Default::default())
    }
}

impl<T> Parser for ParserFromInput<T>
where
    T: FromInput,
{
    type Error = T::Error;
    type Output = T;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        T::parse(input)
    }
}

impl<T> Parser for Option<T>
where
    T: FromInput,
{
    type Error = T::Error;
    type Output = Option<T>;

    fn parse(self, input: &mut Input<'_>) -> Result<Self::Output, Self::Error> {
        T::into_parser().ok().parse(input)
    }
}

/// An extension trait to add `parse` function to [`Input`] stream.
pub trait InputExt {
    fn parse<Item>(&mut self) -> Result<Item, Item::Error>
    where
        Item: FromInput;
}

impl<'a> InputExt for Input<'a> {
    fn parse<Item>(&mut self) -> Result<Item, Item::Error>
    where
        Item: FromInput,
    {
        Item::parse(self)
    }
}

/// The parser ensue the next token is char `c`.
pub fn ensure_char(c: char) -> impl Parser<Output = Span, Error = Kind> + Clone {
    move |input: &mut Input<'_>| {
        let (next, span) = input.next();

        if let Some(next) = next {
            if c == next {
                return Ok(span);
            }

            input.seek(span);

            return Err(Kind::Char(c, span.recoverable()));
        }

        return Err(Kind::Char(c, span.incomplete()));
    }
}

/// A type trait used by [`ensure_keyword`] function.
pub trait Keyword: Clone {
    /// Get `String` representation of this keyword.
    fn into_string(self) -> String;

    /// Returns char iterator.
    fn chars(&self) -> Chars<'_>;

    /// Return keywrod length in bytes.
    fn len(&self) -> usize;
}

impl Keyword for &str {
    fn into_string(self) -> String {
        str::to_string(&self)
    }

    fn chars(&self) -> Chars<'_> {
        str::chars(&self)
    }

    fn len(&self) -> usize {
        str::len(&self)
    }
}

impl Keyword for String {
    fn into_string(self) -> String {
        self
    }

    fn chars(&self) -> Chars<'_> {
        str::chars(&self)
    }

    fn len(&self) -> usize {
        str::len(&self)
    }
}

/// The parser ensue the next token is a keyword `kw`.
pub fn ensure_keyword<KW: Keyword>(kw: KW) -> impl Parser<Output = Span, Error = Kind> + Clone {
    assert!(kw.len() > 0, "keyword length must greate than 0");
    move |input: &mut Input<'_>| {
        let chars = kw.chars();

        let mut start = None;
        let mut end = None;

        for c in chars {
            let (next, span) = input.next();

            if start.is_none() {
                start = Some(span);
            }

            end = Some(span);

            if let Some(next) = next {
                if next != c {
                    return Err(Kind::Keyword(
                        kw.into_string(),
                        start.unwrap().recoverable(),
                    ));
                }
            } else {
                return Err(Kind::Keyword(kw.into_string(), start.unwrap().incomplete()));
            }
        }

        let span = start.unwrap().extend_to_inclusive(end.unwrap());

        Ok(span)
    }
}

/// Returns the longest input [`Span`] (if any) that matches the predicate.
pub fn take_while_indices<F>(f: F) -> impl Parser<Output = Option<Span>, Error = Kind>
where
    F: Fn(usize, char) -> bool,
{
    move |input: &mut Input<'_>| {
        let (c, start) = input.next();

        if c.is_none() {
            return Ok(None);
        }

        let mut indx = 0;

        if !f(indx, c.unwrap()) {
            input.seek(start);
            return Ok(None);
        }

        let mut end = start;

        while let (Some(c), span) = input.next() {
            indx += 1;

            if !f(indx, c) {
                input.seek(span);
                break;
            }

            end = span;
        }

        Ok(Some(start.extend_to_inclusive(end)))
    }
}

/// Returns the longest input [`Span`] (if any) that matches the predicate.
pub fn take_while<F>(f: F) -> impl Parser<Output = Option<Span>, Error = Kind>
where
    F: Fn(char) -> bool,
{
    take_while_indices(move |_, c| f(c))
}

/// Returns the longest input slice (if any) till a predicate is met.
pub fn take_till<F>(f: F) -> impl Parser<Output = Option<Span>, Error = Kind>
where
    F: Fn(char) -> bool,
{
    take_while_indices(move |_, c| !f(c))
}

/// Returns the longest input slice (if any) till a predicate is met.
pub fn take_till_indices<F>(f: F) -> impl Parser<Output = Option<Span>, Error = Kind>
where
    F: Fn(usize, char) -> bool,
{
    take_while_indices(move |idx, c| !f(idx, c))
}

#[cfg(test)]
mod tests {
    use crate::{ensure_char, take_while, Input, Kind, ParserExt, Span, ToDiagnostic};

    use super::{ensure_keyword, Parser};

    #[test]
    fn test_keyword() {
        assert_eq!(
            ensure_keyword("fn").parse(&mut Input::from("fnhello")),
            Ok(Span::new(0, 2, 1, 1))
        );

        assert_eq!(
            ensure_keyword("struct").parse(&mut Input::from("structhello")),
            Ok(Span::new(0, 6, 1, 1))
        );

        assert_eq!(
            ensure_keyword("fn").parse(&mut Input::from("hfnello")),
            Err(Kind::Keyword(
                "fn".to_string(),
                Span::new(0, 1, 1, 1).recoverable()
            ))
        );

        assert_eq!(
            ensure_keyword("fn").parse(&mut Input::from("")),
            Err(Kind::Keyword(
                "fn".to_string(),
                Span::new(0, 0, 1, 1).incomplete()
            ))
        );
    }

    #[test]
    fn test_char() {
        assert_eq!(
            ensure_char('f').parse(&mut Input::from("fnhello")),
            Ok(Span::new(0, 1, 1, 1))
        );

        let mut input = Input::from("hfnello");

        assert_eq!(
            ensure_char('f').parse(&mut input),
            Err(Kind::Char('f', Span::new(0, 1, 1, 1).recoverable()))
        );

        assert_eq!(input.size_hint(), (0, 7));

        let mut input = Input::from("");

        assert_eq!(
            ensure_char('f').parse(&mut input),
            Err(Kind::Char('f', Span::new(0, 0, 1, 1).incomplete()))
        );

        assert_eq!(input.size_hint(), (0, 0));
    }

    #[test]
    fn test_take_while() {
        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut Input::from("hello1")),
            Ok(Some(Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut Input::from("捏啊哈！！")),
            Ok(Some(Span::new(0, 9, 1, 1)))
        );

        let mut input = Input::from("！hello");

        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut input),
            Ok(None)
        );

        let mut input = Input::from("he！llo");

        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut input),
            Ok(Some(Span::new(0, 2, 1, 1)))
        );

        assert_eq!(input.size_hint(), (2, 8));

        let mut input = Input::from("");
        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut input),
            Ok(None)
        );

        assert_eq!(input.size_hint(), (0, 0));
    }

    #[test]
    fn test_ok() {
        assert_eq!(
            ensure_keyword("fn").ok().parse(&mut Input::from("fn")),
            Ok(Some(Span::new(0, 2, 1, 1)))
        );

        assert_eq!(
            ensure_keyword("fn").ok().parse(&mut Input::from("!fn")),
            Ok(None)
        );

        assert_eq!(
            ensure_keyword("fn").ok().parse(&mut Input::from("")),
            Ok(None)
        );

        let mut input = Input::from("ft");

        assert_eq!(ensure_keyword("fn").ok().parse(&mut input), Ok(None));

        assert_eq!(input.size_hint(), (0, 2));
    }

    #[test]
    fn test_map() {
        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .parse(&mut Input::from("fn")),
            Ok(true)
        );

        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .parse(&mut Input::from("!fn")),
            Err(Kind::Keyword(
                "fn".to_string(),
                Span::new(0, 1, 1, 1).recoverable()
            ))
        );

        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .parse(&mut Input::from("")),
            Err(Kind::Keyword(
                "fn".to_string(),
                Span::new(0, 0, 1, 1).incomplete()
            ))
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .or(ensure_keyword("struct").map(|_| false))
                .parse(&mut Input::from("fn")),
            Ok(true)
        );

        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .or(ensure_keyword("struct").map(|_| false))
                .parse(&mut Input::from("struct")),
            Ok(false)
        );
    }
}
