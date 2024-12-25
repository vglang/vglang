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

#[cfg(test)]
mod tests {
    use crate::{
        opcode::{
            attrs::{Fill, Stroke},
            data::Color,
            el::{Characters, Group, Text},
        },
        sexpr::Graphics,
    };

    #[test]
    fn test_apply_children() {
        fn create_text() -> impl Graphics {
            Text::from((10, 100))
                .children(Characters::from("hello world").apply(Fill::from(Color::red)))
        }

        Group
            .apply(Stroke::from(Color::aliceblue))
            .children(create_text());
    }
}
