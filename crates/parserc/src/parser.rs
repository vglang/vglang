use std::{cell::RefCell, ops::RangeTo, rc::Rc};

use crate::Source;

/// All parser combinator must implement this trait.
pub trait Parser: Clone {
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
        F: FnOnce(Self::Output) -> Output,
        Self: Sized,
    {
        Map(self, f)
    }

    /// Create a [`MapRes`] parser.
    fn map_res<F, Output>(self, f: F) -> MapRes<Self, F>
    where
        F: FnOnce(&mut Source<'_>, Self::Output) -> Result<Output, Self::Error>,
        Self: Sized,
    {
        MapRes(self, f)
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

    /// Create a [`Repeat`] parser.
    fn repeat(self, range: RangeTo<usize>) -> Repeat<Self>
    where
        Self: Sized,
    {
        Repeat::new(self, range)
    }
}

impl<T, Error, O> Parser for T
where
    Error: From<crate::Error>,
    for<'a> T: FnOnce(&'a mut Source<'_>) -> Result<O, Error> + Clone + 'static,
{
    type Output = O;

    type Error = Error;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        self(source)
    }
}

/// A wrapper parser that returns `None` when inner parse returns an error.
#[derive(Clone)]
pub struct Optional<T>(T);

impl<T> Parser for Optional<T>
where
    T: Parser,
{
    type Output = Option<T::Output>;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        if let Some(span) = source.span() {
            match self.0.parse(source) {
                Ok(v) => Ok(Some(v)),
                Err(_) => {
                    // rollback.
                    source.seek(span).unwrap();
                    Ok(None)
                }
            }
        } else {
            Ok(None)
        }
    }
}

/// A parser may returns `None`.
#[derive(Clone)]
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
#[derive(Clone)]
pub struct Map<T, F>(T, F);

impl<T, F, Output> Parser for Map<T, F>
where
    T: Parser + 'static,
    for<'a> F: FnOnce(T::Output) -> Output + Clone + 'a,
{
    type Output = Output;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        Ok(self.1(self.0.parse(source)?))
    }
}

/// A wrapper parser that convert inner parse's output to another one according to the `F`.

#[derive(Clone)]
pub struct MapRes<T, F>(T, F);

impl<T, F, Output> Parser for MapRes<T, F>
where
    T: Parser + 'static,
    for<'a> F: FnOnce(&mut Source<'_>, T::Output) -> Result<Output, T::Error> + Clone + 'a,
{
    type Output = Output;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let v = self.0.parse(source)?;
        self.1(source, v)
    }
}

/// A wrapper parser that filter output according to the `F`.
#[derive(Clone)]
pub struct Filter<T, F>(T, F);

impl<T, F> Parser for Filter<T, F>
where
    T: Parser,
    for<'a> F: FnOnce(&T::Output) -> bool + Clone + 'a,
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
#[derive(Clone)]
pub struct FilterMap<T, F>(T, F);

impl<T, F, Output> Parser for FilterMap<T, F>
where
    T: Parser + 'static,
    for<'a> F: FnOnce(T::Output) -> Option<Output> + Clone + 'a,
{
    type Output = Option<Output>;

    type Error = T::Error;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        Ok(self.1(self.0.parse(source)?))
    }
}

/// A wrapper parser that call inner parser N times.
#[derive(Clone)]
pub struct Repeat<T> {
    parser: T,
    range: RangeTo<usize>,
    offset: Rc<RefCell<usize>>,
}

impl<T> Repeat<T> {
    fn new(parser: T, range: RangeTo<usize>) -> Self {
        Self {
            parser,
            range,
            offset: Rc::new(RefCell::new(0)),
        }
    }

    fn has_next(&self) -> bool {
        *self.offset.borrow() < self.range.end
    }
}

impl<T> Iterator for Repeat<T>
where
    T: Clone,
{
    type Item = RepeatItem<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next() {
            Some(RepeatItem(self.clone()))
        } else {
            None
        }
    }
}
#[must_use = "Repeat is a lazy parser iterator, call RepeatItem::parse to do the real parsing."]
#[derive(Clone)]
pub struct RepeatItem<T>(Repeat<T>);

impl<T> Parser for RepeatItem<T>
where
    T: Parser,
{
    type Error = T::Error;
    type Output = T::Output;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Self::Error> {
        let output = self.0.parser.parse(source)?;

        *self.0.offset.borrow_mut() += 1;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, Source};

    #[test]
    fn test_repeat() {
        use crate::{keyword, Parser};

        let mut source = Source::from("fnfnfnfn");

        for parser in keyword("fn").repeat(..3) {
            parser.parse(&mut source).unwrap();
        }

        keyword("fn").parse(&mut source).unwrap();

        assert_eq!(keyword("fn").parse(&mut source), Err(Error::Eof));
    }
}
