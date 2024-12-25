mod graphics;
pub use graphics::*;

mod map_collect;
pub use map_collect::*;

mod el;
pub use el::*;

mod content_of;
pub use content_of::*;

mod apply_to;
pub use apply_to::*;

mod context;
pub use context::*;

vglang_derive::make_tuple_impl!(40);

#[cfg(test)]
mod tests {

    use crate::{
        opcode::{
            attrs::{Fill, Stroke},
            data::Color,
            el::{Characters, For, Group, Text},
        },
        sexpr::{BuildContext, Graphics},
    };

    #[test]
    fn test_apply_children() {
        fn create_text() -> impl Graphics {
            Text::from(((10, 20, 30), 100))
                .children(Characters::from("hello world").apply(Fill::from(Color::red)))
        }

        Group
            .apply((Stroke::from(Color::aliceblue), Fill::default()))
            .children(For::from(0..10).children((
                create_text(),
                create_text(),
                create_text(),
                create_text(),
            )))
            .build(&mut BuildContext::default());
    }
}
