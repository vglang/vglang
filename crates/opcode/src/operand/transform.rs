use std::fmt::Display;

/// A `transform` attribute.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transform {
    Translate {
        tx: f32,
        ty: f32,
    },
    /// compressed 3x3 matrix.
    Matrix {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
    },
    Scale {
        sx: f32,
        sy: Option<f32>,
    },
    Rotate {
        angle: f32,
        cx: f32,
        cy: f32,
    },
    SkewX(f32),
    SkewY(f32),
}

impl Display for Transform {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Transform::Translate { tx, ty } => {
                write!(_f, "translate({},{})", tx, ty)
            }
            Transform::Matrix { a, b, c, d, e, f } => {
                write!(_f, "matrix({},{},{},{},{},{})", a, b, c, d, e, f)
            }
            Transform::Scale { sx, sy } => {
                if let Some(sy) = sy {
                    write!(_f, "scale({},{})", sx, sy)
                } else {
                    write!(_f, "scale({})", sx)
                }
            }
            Transform::Rotate { angle, cx, cy } => write!(_f, "rotate({},{},{})", angle, cx, cy),
            Transform::SkewX(angle) => write!(_f, "skewX({})", angle),
            Transform::SkewY(angle) => write!(_f, "skewY({})", angle),
        }
    }
}

impl Transform {
    /// Create an [`identity matrix`](https://www.wikiwand.com/en/articles/Identity_matrix).
    pub fn identity() -> Self {
        Self::Matrix {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }
}
