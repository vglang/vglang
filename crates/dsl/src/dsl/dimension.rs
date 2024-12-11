use vglang_ir::Measurement;

/// A trait convert self into [`Measurement`]
pub trait MeasurementDsl {
    fn em(self) -> Measurement;

    fn ex(self) -> Measurement;

    fn px(self) -> Measurement;

    fn inch(self) -> Measurement;

    fn cm(self) -> Measurement;

    fn mm(self) -> Measurement;

    fn pt(self) -> Measurement;

    fn pc(self) -> Measurement;

    fn percentage(self) -> Measurement;
}

impl MeasurementDsl for f32 {
    fn em(self) -> Measurement {
        Measurement::em(self)
    }

    fn ex(self) -> Measurement {
        Measurement::ex(self)
    }

    fn px(self) -> Measurement {
        Measurement::px(self)
    }

    fn inch(self) -> Measurement {
        Measurement::inch(self)
    }

    fn cm(self) -> Measurement {
        Measurement::cm(self)
    }

    fn mm(self) -> Measurement {
        Measurement::mm(self)
    }

    fn pt(self) -> Measurement {
        Measurement::pt(self)
    }

    fn pc(self) -> Measurement {
        Measurement::pc(self)
    }

    fn percentage(self) -> Measurement {
        Measurement::percentage(self)
    }
}

impl MeasurementDsl for i32 {
    fn em(self) -> Measurement {
        Measurement::em(self as f32)
    }

    fn ex(self) -> Measurement {
        Measurement::ex(self as f32)
    }

    fn px(self) -> Measurement {
        Measurement::px(self as f32)
    }

    fn inch(self) -> Measurement {
        Measurement::inch(self as f32)
    }

    fn cm(self) -> Measurement {
        Measurement::cm(self as f32)
    }

    fn mm(self) -> Measurement {
        Measurement::mm(self as f32)
    }

    fn pt(self) -> Measurement {
        Measurement::pt(self as f32)
    }

    fn pc(self) -> Measurement {
        Measurement::pc(self as f32)
    }

    fn percentage(self) -> Measurement {
        Measurement::percentage(self as f32)
    }
}
