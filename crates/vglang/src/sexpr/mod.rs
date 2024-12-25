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
    use crate::opcode::{
        attrs::Stroke,
        el::{Characters, Group, Text},
    };

    #[test]
    fn test_apply_children() {
        Group
            .apply(Stroke::default())
            .children(Text::default().children(Characters::from("hello world")));
    }
}
