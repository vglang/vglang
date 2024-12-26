use vglang::{
    opcode::{
        attrs::*,
        data::*,
        el::*,
        variable::{Target, Variable},
    },
    sexpr::*,
};

#[test]
fn test_path() {
    Path::from((
        move_to(300, 200),
        line_to(150, 200),
        arc(150, 150, 0, true, false, (300, 50)),
        close(),
    ))
    .build(&mut BuildContext::default());
}

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

#[test]
fn test_control_flow() {
    Foreach::from("test").children(Text::from((10, 20)));

    If(Variable::from(("", Target::Register)))
        .children(Text::from((10, 20)))
        .Else(Text::from((20, 10)));

    Group.children((
        Text::from((10, 20)),
        Foreach::from("hello").children(Characters::from("hello world")),
    ));
}

#[test]
fn test_canvas() {
    Canvas::from((10, 20))
        .apply(ViewBox::from((0, 0, 100, 200)))
        .children(Text::default().children((
            TextSpan::default().children(Characters::from("hello world")),
            TextSpan::default().children(Characters::from("hello world")),
        )))
        .build(&mut BuildContext::default());
}
