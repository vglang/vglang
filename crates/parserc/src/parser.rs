use std::ops::{Range, RangeTo};

use crate::{Error, Source};

/// All parser combinator must implement this trait.
pub trait Parser {
    /// The output returns by [`parse`](Parser::parse) fn.
    type Output;

    /// Error type that this trait may returns.
    type Error: From<crate::Error>;

    /// Consume source and parse next syntax item.
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error>;

    /// Create an [`Optional`] parser.
    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional(self)
    }

    /// Create an [`Or`] parser.
    fn or<T>(self, rhs: T) -> Or<Self, T>
    where
        Self: Sized,
    {
        Or(self, rhs)
    }

    /// Create a [`Map`] parser.
    fn map<F, Output>(self, f: F) -> Map<Self, F>
    where
        for<'a> F: FnOnce(Self::Output) -> Output,
        Self: Sized,
    {
        Map(self, f)
    }

    /// Create a [`Filter`] parser.
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        for<'a> F: FnOnce(&Self::Output) -> bool,
        Self: Sized,
    {
        Filter(self, f)
    }

    /// Create a [`FilterMap`] parser.
    fn filter_map<F, Output>(self, f: F) -> FilterMap<Self, F>
    where
        for<'a> F: FnOnce(Self::Output) -> Option<Output>,
        Self: Sized,
    {
        FilterMap(self, f)
    }

    /// Create a [`RangeParser`] parser.
    fn range(self, range: Range<usize>) -> RangeParser<Self>
    where
        Self: Sized,
    {
        RangeParser(self, range)
    }

    /// Create a [`RangeToParser`] parser.
    fn range_to(self, range: RangeTo<usize>) -> RangeToParser<Self>
    where
        Self: Sized,
    {
        RangeToParser(self, range)
    }

    /// Create a [`Repeat`] parser.
    fn repeat(self, times: usize) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat(self, times)
    }
}

impl<T, Error, O> Parser for T
where
    Error: From<crate::Error>,
    for<'a> T: FnOnce(&'a mut Source<'_>) -> Result<O, Error> + 'static,
{
    type Output = O;

    type Error = Error;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        self(source)
    }
}

/// A wrapper parser that returns `None` when inner parse returns an error.
pub struct Optional<T>(T);

impl<T> Parser for Optional<T>
where
    T: Parser,
{
    type Output = Option<T::Output>;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let span = source.span();

        match self.0.parse(source) {
            Ok(v) => Ok(Some(v)),
            Err(_) => {
                // rollback.
                source.seek(span).unwrap();
                Ok(None)
            }
        }
    }
}

/// A parser may returns `None`.
pub struct Or<L, R>(L, R);

impl<L, R, E> Parser for Or<L, R>
where
    L: Parser<Error = E>,
    for<'a> R: Parser<Output = L::Output, Error = E> + 'a,
    E: From<crate::Error>,
{
    type Output = L::Output;

    type Error = E;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let lhs = self.0.optional().parse(source)?;

        if let Some(v) = lhs {
            return Ok(v);
        } else {
            self.1.parse(source)
        }
    }
}

/// A wrapper parser that convert inner parse's output to another one according to the `F`.
pub struct Map<T, F>(T, F);

impl<T, F, Output> Parser for Map<T, F>
where
    T: Parser + 'static,
    for<'a> F: FnOnce(T::Output) -> Output + 'a,
{
    type Output = Output;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        Ok(self.1(self.0.parse(source)?))
    }
}

/// A wrapper parser that filter output according to the `F`.
pub struct Filter<T, F>(T, F);

impl<T, F> Parser for Filter<T, F>
where
    T: Parser,
    for<'a> F: FnOnce(&T::Output) -> bool + 'a,
{
    type Output = Option<T::Output>;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let v = self.0.parse(source)?;

        if self.1(&v) {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }
}

/// A wrapper parser that filter inner parser output and map it to a different type according to the `F`.
pub struct FilterMap<T, F>(T, F);

impl<T, F, Output> Parser for FilterMap<T, F>
where
    T: Parser + 'static,
    for<'a> F: FnOnce(T::Output) -> Option<Output> + 'a,
{
    type Output = Option<Output>;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        Ok(self.1(self.0.parse(source)?))
    }
}

/// A range(N..M) parser that call inner parser at least N times, and at most M times.
pub struct RangeParser<T>(T, Range<usize>);

impl<T> Parser for RangeParser<T>
where
    T: Parser + Clone,
{
    type Output = Vec<T::Output>;

    type Error = T::Error;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let mut output = vec![];
        for i in 0..self.1.end {
            if let Some(v) = self.0.clone().optional().parse(source)? {
                output.push(v);
            } else {
                if i < self.1.start {
                    return Err(Error::Range(self.1).into());
                }
            }
        }

        return Ok(output);
    }
}

/// A range_to(..M) parser that call inner parser at most M times.
pub struct RangeToParser<T>(T, RangeTo<usize>);

impl<T> Parser for RangeToParser<T>
where
    T: Parser + Clone,
{
    type Output = Vec<T::Output>;

    type Error = T::Error;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let mut output = vec![];
        for _ in 0..self.1.end {
            if let Some(v) = self.0.clone().optional().parse(source)? {
                output.push(v);
            }
        }

        return Ok(output);
    }
}

/// A wrapper parser that call inner parser N times.
pub struct Repeat<T>(T, usize);

impl<T> Parser for Repeat<T>
where
    T: Parser + Clone,
{
    type Output = Vec<T::Output>;

    type Error = T::Error;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let mut output = vec![];
        for _ in 0..self.1 {
            if let Some(v) = self.0.clone().optional().parse(source)? {
                output.push(v);
            } else {
                return Err(Error::Repeat(self.1).into());
            }
        }

        return Ok(output);
    }
}
