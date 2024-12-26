use crate::opcode::data::{Length, Number};

/// A trait convert self into [`Length`]
pub trait Slength {
    fn em(self) -> Length;

    fn ex(self) -> Length;

    fn px(self) -> Length;

    fn r#in(self) -> Length;

    fn cm(self) -> Length;

    fn mm(self) -> Length;

    fn pt(self) -> Length;

    fn pc(self) -> Length;

    fn percent(self) -> Length;
}

impl<T> Slength for T
where
    Number: From<T>,
{
    fn em(self) -> Length {
        Length::em(Number::from(self).0)
    }

    fn ex(self) -> Length {
        Length::ex(Number::from(self).0)
    }

    fn px(self) -> Length {
        Length::px(Number::from(self).0)
    }

    fn r#in(self) -> Length {
        Length::r#in(Number::from(self).0)
    }

    fn cm(self) -> Length {
        Length::cm(Number::from(self).0)
    }

    fn mm(self) -> Length {
        Length::mm(Number::from(self).0)
    }

    fn pt(self) -> Length {
        Length::pt(Number::from(self).0)
    }

    fn pc(self) -> Length {
        Length::pc(Number::from(self).0)
    }

    fn percent(self) -> Length {
        Length::percent(Number::from(self).0)
    }
}
