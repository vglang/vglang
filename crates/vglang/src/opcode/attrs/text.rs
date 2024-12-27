use crate::opcode::{
    data::{
        AlignmentBaseline, BaselineShift, DominantBaseline, LetterSpacing, TextAnchor,
        TextDecoration, TextDirection, UnicodeBidi, WordSpacing, WritingMode,
    },
    variable::Variable,
};

#[cfg(feature = "sexpr")]
use crate::opcode::el::*;

/// support for various international writing directions, such as left-to-right (e.g., Latin scripts) and
/// bidirectional (e.g., Hebrew or Arabic) and vertical (e.g., Asian scripts).
#[derive(Debug, Default, PartialEq, PartialOrd, Clone)]
#[cfg_attr(
    feature = "sexpr",
    vglang_derive::attribute(
        boxed, Group, Text, TextSpan, Characters, Rect, Circle, Line, Polyline, Polygon
    )
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextLayout {
    /// See [`WritingMode`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_mode: Option<WritingMode>,
    /// See [`TextDirection`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<TextDirection>,

    /// See [`UnicodeBidi`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_bidi: Option<UnicodeBidi>,

    /// See [`TextAnchor`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Variable<TextAnchor>>,

    /// See [`DominantBaseline`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dominant_baseline: Option<Variable<DominantBaseline>>,

    /// See [`AlignmentBaseline`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment_baseline: Option<Variable<AlignmentBaseline>>,

    /// See [`BaselineShift`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline_shift: Option<Variable<BaselineShift>>,

    /// See [`TextDecoration`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoration: Option<Variable<TextDecoration>>,

    /// See [`LetterSpacing`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub letter_spacing: Option<Variable<LetterSpacing>>,

    /// See [`WordSpacing`]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub word_spacing: Option<Variable<WordSpacing>>,
}

impl From<WritingMode> for TextLayout {
    fn from(value: WritingMode) -> Self {
        Self {
            write_mode: Some(value),
            ..Default::default()
        }
    }
}

impl From<TextDirection> for TextLayout {
    fn from(value: TextDirection) -> Self {
        Self {
            direction: Some(value),
            ..Default::default()
        }
    }
}

impl From<UnicodeBidi> for TextLayout {
    fn from(value: UnicodeBidi) -> Self {
        Self {
            unicode_bidi: Some(value),
            ..Default::default()
        }
    }
}

impl From<TextAnchor> for TextLayout {
    fn from(value: TextAnchor) -> Self {
        Self {
            anchor: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<DominantBaseline> for TextLayout {
    fn from(value: DominantBaseline) -> Self {
        Self {
            dominant_baseline: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<AlignmentBaseline> for TextLayout {
    fn from(value: AlignmentBaseline) -> Self {
        Self {
            alignment_baseline: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<BaselineShift> for TextLayout {
    fn from(value: BaselineShift) -> Self {
        Self {
            baseline_shift: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<TextDecoration> for TextLayout {
    fn from(value: TextDecoration) -> Self {
        Self {
            decoration: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<LetterSpacing> for TextLayout {
    fn from(value: LetterSpacing) -> Self {
        Self {
            letter_spacing: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}

impl From<WordSpacing> for TextLayout {
    fn from(value: WordSpacing) -> Self {
        Self {
            word_spacing: Some(Variable::Constant(value)),
            ..Default::default()
        }
    }
}
