use vglang_opcode::operand::Length;

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

impl Slength for f32 {
    fn em(self) -> Length {
        Length::em(self)
    }

    fn ex(self) -> Length {
        Length::ex(self)
    }

    fn px(self) -> Length {
        Length::px(self)
    }

    fn r#in(self) -> Length {
        Length::r#in(self)
    }

    fn cm(self) -> Length {
        Length::cm(self)
    }

    fn mm(self) -> Length {
        Length::mm(self)
    }

    fn pt(self) -> Length {
        Length::pt(self)
    }

    fn pc(self) -> Length {
        Length::pc(self)
    }

    fn percent(self) -> Length {
        Length::percent(self)
    }
}

impl Slength for i32 {
    fn em(self) -> Length {
        Length::em(self as f32)
    }

    fn ex(self) -> Length {
        Length::ex(self as f32)
    }

    fn px(self) -> Length {
        Length::px(self as f32)
    }

    fn r#in(self) -> Length {
        Length::r#in(self as f32)
    }

    fn cm(self) -> Length {
        Length::cm(self as f32)
    }

    fn mm(self) -> Length {
        Length::mm(self as f32)
    }

    fn pt(self) -> Length {
        Length::pt(self as f32)
    }

    fn pc(self) -> Length {
        Length::pc(self as f32)
    }

    fn percent(self) -> Length {
        Length::percent(self as f32)
    }
}
