mod dsl;
use dsl::svg;
use vglang_dsl::{
    attrs::*,
    drawing::{apply, layer, with},
};

#[futures_test::test]
async fn test_text() {
    svg(
        "basic",
        layer(
            Layer::default()
                .width(Measurement::cm(10.0))
                .height(Measurement::cm(3.0))
                .viewbox((0.0, 0.0, 1000.0, 300.0, PreserveAspectRatio::default())),
            (
                // apply `fill`,`stroke` to `rect` element.
                apply(
                    (Fill::default(), Stroke::default().paint(Color::blue)),
                    Rect::default().x(1).y(1).width(998).height(298).rx(20),
                ),
                apply(
                    // apply font selection properties
                    (
                        Font::from(FontFamily::Monospace)
                            .size(50)
                            .style(FontStyle::Italic),
                        TextLayout::from(TextAnchor::Middle)
                            .alignment_baseline(AlignmentBaseline::Hanging),
                    ),
                    (
                        with(Text::default().x(500).y(150), "Hello cotati. 你好"),
                        // override `variant`, `weight` properties.
                        apply(
                            (
                                Font::from(FontVariant::Normal)
                                    .weight(FontWeight::Bolder)
                                    .size(30)
                                    .stretch(FontStretch::UltraCondensed),
                                TextLayout::from(TextAnchor::Middle)
                                    .write_mode(vglang_ir::WritingMode::TbRl)
                                    .vertical(vglang_ir::GlyphOrientationVertical::Angle(
                                        90.into(),
                                    )),
                            ),
                            with(Text::default().x(900).y(150), "你好 Hello VGL."),
                        ),
                    ),
                ),
            ),
        ),
    )
    .await;
}
