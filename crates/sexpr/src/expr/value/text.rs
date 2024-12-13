use super::Svalue;

/// See [`length_adjust`](Text::length_adjust)
#[derive(Debug, PartialEq, PartialOrd, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StextLengthAdjust {
    /// 'spacing' indicates that only the advance values are adjusted. The glyphs themselves are not stretched or compressed.
    Spacing,

    /// 'spacingAndGlyphs' indicates that the advance values are adjusted and the glyphs themselves stretched or compressed
    /// in one axis (i.e., a direction parallel to the inline-progression-direction).
    SpacingAndGlyphs,
}

impl Default for StextLengthAdjust {
    fn default() -> Self {
        Self::Spacing
    }
}

impl From<StextLengthAdjust> for Svalue {
    fn from(value: StextLengthAdjust) -> Self {
        Self::TextLengthAdjust(value)
    }
}

impl TryFrom<Svalue> for StextLengthAdjust {
    type Error = Svalue;
    fn try_from(value: Svalue) -> Result<Self, Self::Error> {
        match value {
            Svalue::TextLengthAdjust(v) => Ok(v),
            _ => Err(value),
        }
    }
}
