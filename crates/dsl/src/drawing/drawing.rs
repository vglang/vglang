use cotati_ir::IR;

use crate::generator::Generator;

/// A `Graphic` element defines a group of vector graphic instructions.
///
/// Consumes this object via [`draw`](Graphic::draw) to generate real drawing instructions.
pub trait Graphic<G>
where
    G: Generator,
{
    /// Consume self and generate real drawing instructions.
    fn draw(self, g: &mut G);
}

/// Implements [`Graphic`] for functions with signature `FnOnce(&mut G)`.
impl<G, F> Graphic<G> for F
where
    F: FnOnce(&mut G),
    G: Generator,
{
    fn draw(self, g: &mut G) {
        self(g)
    }
}

impl<G> Graphic<G> for String
where
    G: Generator,
{
    fn draw(self, g: &mut G) {
        g.push(IR::String(self));
    }
}

impl<G> Graphic<G> for &str
where
    G: Generator,
{
    fn draw(self, g: &mut G) {
        g.push(IR::String(self.to_owned()));
    }
}

/// Create a graphic element that create a reference instruction to animatable register.
pub fn animated<G: Generator, S: ToOwned<Owned = String>>(name: S) -> impl Graphic<G> {
    move |g: &mut G| {
        g.push(IR::Animated(name.to_owned()));
    }
}

macro_rules! tuple_drawing {
    ($header: ident, $($tail: ident),+) => {

        impl<$header, $($tail),+ , G> Graphic<G> for ($header, $($tail),+)
        where
            G: Generator,
            $header: Graphic<G>,
            $($tail: Graphic<G>),+,
        {
            #[allow(non_snake_case)]
            fn draw(self, render: &mut G) {
                let ($header, $($tail),+) = self;
                $header.draw(render);
                $($tail.draw(render);)+
            }
        }

        tuple_drawing!($($tail),+);
    };
    ($header: ident) => {}
}

tuple_drawing!(
    A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16, A17, A18, A19, A20
);
