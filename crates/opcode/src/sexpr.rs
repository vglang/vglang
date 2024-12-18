/// Map item via iterator and collect them into vec.
pub trait MapCollect<Item> {
    fn map_collect(self) -> Vec<Item>;
}

impl<F, T> MapCollect<T> for F
where
    T: From<F>,
{
    fn map_collect(self) -> Vec<T> {
        vec![self.into()]
    }
}

macro_rules! tuple_map_collect {
    ($item: ident, $header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+> $crate::MapCollect<$item> for ($header, $($tail),+)
        where
            $header: Into<$item>,
            $($tail: Into<$item>),+,
        {
            #[allow(non_snake_case)]
            fn map_collect(self) -> Vec<$item> {
                let ($header, $($tail),+) = self;
                vec![$header.into(),$($tail.into()),+]
            }
        }

        tuple_map_collect!($item, $($tail),+);
    };
    ($item: ident, $header: ident) => {}
}

pub(crate) use tuple_map_collect;
