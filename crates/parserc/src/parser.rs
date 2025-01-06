use std::{marker::PhantomData, str::Chars};

use crate::{ControlFlow, Kind, ParseContext, Result, Span};

/// A parser produce output by parsing and consuming the [`Input`] char stream.
pub trait Parser<'a, C>
where
    C: ParseContext<'a>,
{
    /// Output data type.
    type Output;

    /// Parse and generate a new output.
    fn parse(self, input: &mut C) -> Result<Self::Output>;
}

/// Implement [`Parser`] for all [`FnMut`](&mut Input<'_>) -> Result<O, E>.
impl<'a, C, F, O> Parser<'a, C> for F
where
    C: ParseContext<'a>,
    F: FnOnce(&mut C) -> Result<O>,
{
    type Output = O;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
        (self)(input)
    }
}

/// A combinator for [`ok`](ParserExt::ok) function.
#[derive(Clone)]
pub struct Optional<S>(S);

impl<'a, C, S> Parser<'a, C> for Optional<S>
where
    S: Parser<'a, C>,
    C: ParseContext<'a>,
{
    type Output = Option<S::Output>;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
        let start = input.span();
        match self.0.parse(input) {
            Err(err) => match err {
                ControlFlow::Fatal => Err(err),
                ControlFlow::Incomplete | ControlFlow::Recoverable => {
                    input.seek(start);
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

impl<'a, C, S, O, Output> Parser<'a, C> for Or<S, O>
where
    S: Parser<'a, C, Output = Output> + Clone,
    O: Parser<'a, C, Output = Output>,
    C: ParseContext<'a>,
{
    type Output = Output;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
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

impl<'a, C, S, F, U> Parser<'a, C> for Map<S, F>
where
    S: Parser<'a, C>,
    F: FnOnce(S::Output) -> U,
    C: ParseContext<'a>,
{
    type Output = U;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
        self.0.parse(input).map(self.1)
    }
}

/// A combinator for [`map_err`](ParserExt::map_err) function.
#[derive(Clone)]
pub struct MapErr<S, F>(S, F);

impl<'a, C, S, F> Parser<'a, C> for MapErr<S, F>
where
    C: ParseContext<'a>,
    S: Parser<'a, C>,
    F: FnOnce(C::Error, Span) -> (C::Error, Span),
{
    type Output = S::Output;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
        match self.0.parse(input) {
            Err(c) => {
                input.map_err(self.1);
                return Err(c);
            }
            r => r,
        }
    }
}

/// An extension trait for [`Parser`] combinators.
pub trait ParserExt<'a, C>: Parser<'a, C>
where
    C: ParseContext<'a>,
{
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
        F: FnOnce(C::Error) -> C::Error,
        Self: Sized,
    {
        MapErr(self, op)
    }
}

impl<'a, C, T> ParserExt<'a, C> for T
where
    T: Parser<'a, C>,
    C: ParseContext<'a>,
{
}

/// All types that can be parsed from the [`Input`] stream must implement this trait.
///
/// See [`parse`](InputExt::parse) function.
pub trait FromInput<'a, C>
where
    C: ParseContext<'a>,
{
    /// Parse and create `Self` from [`Input`] stream.
    fn parse(input: &mut C) -> Result<Self>
    where
        Self: Sized;
}

/// A helper trait that convert [`FromInput`] into a [`Parser`].
pub trait IntoParser<'a, C>: FromInput<'a, C>
where
    C: ParseContext<'a>,
{
    /// Conver self into parser.
    fn into_parser() -> ParserFromInput<Self>
    where
        Self: Sized,
    {
        ParserFromInput(Default::default())
    }
}

impl<'a, C, T> IntoParser<'a, C> for T
where
    T: FromInput<'a, C>,
    C: ParseContext<'a>,
{
}

/// A wrapper parser for [`FromInput`] type.
pub struct ParserFromInput<T>(PhantomData<T>);

impl<T> Clone for ParserFromInput<T> {
    fn clone(&self) -> Self {
        Self(Default::default())
    }
}

impl<'a, C, T> Parser<'a, C> for ParserFromInput<T>
where
    T: FromInput<'a, C>,
    C: ParseContext<'a>,
{
    type Output = T;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
        T::parse(input)
    }
}

impl<'a, C, T> Parser<'a, C> for Option<T>
where
    T: FromInput<'a, C>,
    C: ParseContext<'a>,
{
    type Output = Option<T>;

    fn parse(self, input: &mut C) -> Result<Self::Output> {
        T::into_parser().ok().parse(input)
    }
}

/// An extension trait to add `parse` function to [`Input`] stream.
pub trait InputExt<'a>: ParseContext<'a> {
    fn parse<Item>(&mut self) -> Result<Item>
    where
        Item: FromInput<'a, Self>,
        Self: Sized;
}

impl<'a, T> InputExt<'a> for T
where
    T: ParseContext<'a>,
{
    fn parse<Item>(&mut self) -> Result<Item>
    where
        Item: FromInput<'a, Self>,
        Self: Sized,
    {
        Item::parse(self)
    }
}

/// The parser ensue the next token is char `c`.
pub fn ensure_char<'a, C>(c: char) -> impl Parser<'a, C, Output = Span> + Clone
where
    C: ParseContext<'a>,
{
    move |input: &mut C| {
        let (next, span) = input.next();

        if let Some(next) = next {
            if c == next {
                return Ok(span);
            }

            input.report_err(Kind::Char(c).into(), span);

            return Err(ControlFlow::Recoverable);
        }
        input.report_err(Kind::Char(c).into(), span);
        return Err(ControlFlow::Incomplete);
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
///
/// A keyword is a seqence of chars without spaces.
pub fn ensure_keyword<'a, C, KW: Keyword>(kw: KW) -> impl Parser<'a, C, Output = Span> + Clone
where
    C: ParseContext<'a>,
{
    assert!(kw.len() > 0, "keyword length must greate than 0");
    move |input: &mut C| {
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
                    input.report_err(Kind::Keyword(kw.into_string()).into(), start.unwrap());

                    return Err(ControlFlow::Recoverable);
                }
            } else {
                input.report_err(Kind::Keyword(kw.into_string()).into(), start.unwrap());
                return Err(ControlFlow::Incomplete);
            }
        }

        let span = start.unwrap().extend_to_inclusive(end.unwrap());

        Ok(span)
    }
}

/// Returns the longest input [`Span`] (if any) that matches the predicate.
pub fn take_while_indices<'a, C, F>(f: F) -> impl Parser<'a, C, Output = Option<Span>>
where
    C: ParseContext<'a>,
    F: Fn(usize, char) -> bool,
{
    move |input: &mut C| {
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
pub fn take_while<'a, C, F>(f: F) -> impl Parser<'a, C, Output = Option<Span>>
where
    C: ParseContext<'a>,
    F: Fn(char) -> bool,
{
    take_while_indices(move |_, c| f(c))
}

/// Returns the longest input slice (if any) till a predicate is met.
pub fn take_till<'a, C, F>(f: F) -> impl Parser<'a, C, Output = Option<Span>>
where
    C: ParseContext<'a>,
    F: Fn(char) -> bool,
{
    take_while_indices(move |_, c| !f(c))
}

/// Returns the longest input slice (if any) till a predicate is met.
pub fn take_till_indices<'a, C, F>(f: F) -> impl Parser<'a, C, Output = Option<Span>>
where
    C: ParseContext<'a>,
    F: Fn(usize, char) -> bool,
{
    take_while_indices(move |idx, c| !f(idx, c))
}

#[cfg(test)]
mod tests {
    use crate::{ensure_char, take_while, ControlFlow, Input, Kind, ParserExt, Source, Span};

    use super::{ensure_keyword, Parser};

    #[test]
    fn test_keyword() {
        assert_eq!(
            ensure_keyword("fn").parse(&mut Input::<Kind>::from("fnhello")),
            Ok(Span::new(0, 2, 1, 1))
        );

        assert_eq!(
            ensure_keyword("struct").parse(&mut Input::<Kind>::from("structhello")),
            Ok(Span::new(0, 6, 1, 1))
        );

        assert_eq!(
            ensure_keyword("fn").parse(&mut Input::<Kind>::from("hfnello")),
            Err(ControlFlow::Recoverable)
        );

        assert_eq!(
            ensure_keyword("fn").parse(&mut Input::<Kind>::from("")),
            Err(ControlFlow::Incomplete)
        );
    }

    #[test]
    fn test_char() {
        assert_eq!(
            ensure_char('f').parse(&mut Input::<Kind>::from("fnhello")),
            Ok(Span::new(0, 1, 1, 1))
        );

        let mut input = Input::<Kind>::from("hfnello");

        assert_eq!(
            ensure_char('f').parse(&mut input),
            Err(ControlFlow::Recoverable)
        );

        assert_eq!(input.size_hint(), (1, 7));

        let mut input = Input::<Kind>::from("");

        assert_eq!(
            ensure_char('f').parse(&mut input),
            Err(ControlFlow::Incomplete)
        );

        assert_eq!(input.size_hint(), (0, 0));
    }

    #[test]
    fn test_take_while() {
        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut Input::<Kind>::from("hello1")),
            Ok(Some(Span::new(0, 5, 1, 1)))
        );

        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut Input::<Kind>::from("捏啊哈！！")),
            Ok(Some(Span::new(0, 9, 1, 1)))
        );

        let mut input = Input::<Kind>::from("！hello");

        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut input),
            Ok(None)
        );

        let mut input = Input::<Kind>::from("he！llo");

        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut input),
            Ok(Some(Span::new(0, 2, 1, 1)))
        );

        assert_eq!(input.size_hint(), (2, 8));

        let mut input = Input::<Kind>::from("");
        assert_eq!(
            take_while(|c| c.is_alphabetic()).parse(&mut input),
            Ok(None)
        );

        assert_eq!(input.size_hint(), (0, 0));
    }

    #[test]
    fn test_ok() {
        assert_eq!(
            ensure_keyword("fn")
                .ok()
                .parse(&mut Input::<Kind>::from("fn")),
            Ok(Some(Span::new(0, 2, 1, 1)))
        );

        assert_eq!(
            ensure_keyword("fn")
                .ok()
                .parse(&mut Input::<Kind>::from("!fn")),
            Ok(None)
        );

        assert_eq!(
            ensure_keyword("fn")
                .ok()
                .parse(&mut Input::<Kind>::from("")),
            Ok(None)
        );

        let mut input = Input::<Kind>::from("ft");

        assert_eq!(ensure_keyword("fn").ok().parse(&mut input), Ok(None));

        assert_eq!(input.size_hint(), (0, 2));
    }

    #[test]
    fn test_map() {
        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .parse(&mut Input::<Kind>::from("fn")),
            Ok(true)
        );

        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .parse(&mut Input::<Kind>::from("!fn")),
            Err(ControlFlow::Recoverable)
        );

        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .parse(&mut Input::<Kind>::from("")),
            Err(ControlFlow::Incomplete)
        );
    }

    #[test]
    fn test_or() {
        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .or(ensure_keyword("struct").map(|_| false))
                .parse(&mut Input::<Kind>::from("fn")),
            Ok(true)
        );

        assert_eq!(
            ensure_keyword("fn")
                .map(|_| true)
                .or(ensure_keyword("struct").map(|_| false))
                .parse(&mut Input::<Kind>::from("struct")),
            Ok(false)
        );
    }
}
