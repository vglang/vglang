use std::{cell::RefCell, fmt::Debug, marker::PhantomData, ops::RangeTo, rc::Rc};

use crate::{Error, Kind, Source};

/// All parser combinator must implement this trait.
pub trait Parser: Clone {
    /// The output returns by [`parse`](Parser::parse) fn.
    type Output;

    /// Error type that this trait may returns.
    type Kind: From<crate::Kind> + PartialEq + Debug;

    /// Consume source and parse next syntax item.
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>>;

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
        F: FnOnce(&mut Source<'_>, Self::Output) -> Result<Output, Error<Self::Kind>>,
        Self: Sized,
    {
        MapRes(self, f)
    }

    /// Create a [`MapError`] parser.
    fn map_error<E>(self) -> MapError<Self, E> {
        MapError(self, Default::default())
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

impl<T, K, O> Parser for T
where
    K: From<crate::Kind> + PartialEq + Debug,
    for<'a> T: FnOnce(&'a mut Source<'_>) -> Result<O, Error<K>> + Clone + 'static,
{
    type Output = O;

    type Kind = K;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
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

    type Kind = T::Kind;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
        match self.0.parse(source) {
            Ok(v) => Ok(Some(v)),
            Err(Error::Recoverable(_, span)) => {
                // rollback.
                source.seek(span);
                Ok(None)
            }
            Err(Error::Incomplete(_)) => Ok(None),

            Err(err) => return Err(err),
        }
    }
}

/// A parser may returns `None`.
#[derive(Clone)]
pub struct Or<L, R>(L, R);

impl<L, R, K> Parser for Or<L, R>
where
    K: From<crate::Kind> + PartialEq + Debug,
    L: Parser<Kind = K>,
    for<'a> R: Parser<Output = L::Output, Kind = K> + 'a,
{
    type Output = L::Output;

    type Kind = K;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
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

    type Kind = T::Kind;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
        Ok(self.1(self.0.parse(source)?))
    }
}

/// A wrapper parser that convert inner parse's output to another one according to the `F`.

#[derive(Clone)]
pub struct MapRes<T, F>(T, F);

impl<T, F, Output> Parser for MapRes<T, F>
where
    T: Parser + 'static,
    for<'a> F: FnOnce(&mut Source<'_>, T::Output) -> Result<Output, Error<T::Kind>> + Clone + 'a,
{
    type Output = Output;

    type Kind = T::Kind;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
        let v = self.0.parse(source)?;
        self.1(source, v)
    }
}

/// A wrapper parser that convert inner parse's output to another one according to the `F`.

pub struct MapError<T, E>(T, PhantomData<E>);

impl<T, E> Clone for MapError<T, E>
where
    T: Parser + 'static,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), Default::default())
    }
}

impl<T, E> Parser for MapError<T, E>
where
    T: Parser + 'static,
    E: From<T::Kind> + From<Kind> + PartialEq + Debug,
{
    type Output = T::Output;

    type Kind = E;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
        match self.0.parse(source) {
            Ok(v) => Ok(v),
            Err(err) => match err {
                Error::Fatal(k, span) => return Err(Error::Fatal(k.into(), span)),
                Error::Recoverable(k, span) => return Err(Error::Recoverable(k.into(), span)),
                Error::Incomplete(k) => return Err(Error::Incomplete(k.into())),
            },
        }
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

    type Kind = T::Kind;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
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

    type Kind = T::Kind;
    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
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
    type Kind = T::Kind;
    type Output = T::Output;

    fn parse(self, source: &mut Source) -> Result<Self::Output, Error<Self::Kind>> {
        let output = self.0.parser.parse(source)?;

        *self.0.offset.borrow_mut() += 1;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Kind, Source};

    #[test]
    fn test_repeat() {
        use crate::{keyword, Parser};

        let mut source = Source::from("fnfnfnfn");

        for parser in keyword("fn").repeat(..3) {
            parser.parse(&mut source).unwrap();
        }

        keyword("fn").parse(&mut source).unwrap();

        assert_eq!(
            keyword("fn").parse(&mut source),
            Err(crate::Error::Incomplete(Kind::Keyword("fn")))
        );
    }
}
