/// see [`svg`] document for more information.
///
/// [`svg`]: https://www.w3.org/TR/SVG11/coords.html#PreserveAspectRatioAttribute
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MeetOrSlice {
    Meet,
    Slice,
}

/// In some cases, typically when using the ‘viewBox’ attribute, i
/// t is desirable that the graphics stretch to fit non-uniformly
/// to take up the entire viewport. In other cases, it is desirable
/// that uniform scaling be used for the purposes of preserving
/// the aspect ratio of the graphics.
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreserveAspectRatio {
    /// Force uniform scaling
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMinYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMidYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>` of the element's ‘viewBox’ with the smallest Y value of the viewport.
    xMaxYMin(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMinYMid(MeetOrSlice),
    /// Force uniform scaling(the default).
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMidYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the midpoint Y value of the element's ‘viewBox’ with the midpoint Y value of the viewport.
    xMaxYMid(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>` of the element's ‘viewBox’ with the smallest X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMinYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the midpoint X value of the element's ‘viewBox’ with the midpoint X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMidYMax(MeetOrSlice),
    /// Force uniform scaling.
    ///
    /// Align the `<min-x>`+`<width>` of the element's ‘viewBox’ with the maximum X value of the viewport.
    /// Align the `<min-y>`+`<height>` of the element's ‘viewBox’ with the maximum Y value of the viewport.
    xMaxYMax(MeetOrSlice),
}

impl Default for PreserveAspectRatio {
    fn default() -> Self {
        Self::xMidYMid(MeetOrSlice::Meet)
    }
}
