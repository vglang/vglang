//! Define the syntax of `vglang` by `mlang`.

use mlang::opcode::{Data, Enum, Field, Opcode, Type};

static ANGLE: &str = "Angle";
static RGB: &str = "Rgb";
static LENGTH: &str = "Length";
static EM: &str = "em";
static EX: &str = "ex";
static PX: &str = "px";
static INCH: &str = "inch";
static CM: &str = "cm";
static MM: &str = "mm";
static PT: &str = "pt";
static PC: &str = "pc";
static PERCENT: &str = "percent";
// static R: &str = "r";
// static G: &str = "g";
// static B: &str = "b";
static IRI: &str = "Iri";
static FUNC_IRI: &str = "FuncIri";
static PATH: &str = "Path";
static POINT: &str = "Point";
static PAINT: &str = "Paint";
static COLOR: &str = "Color";
static SERVER: &str = "Server";
static NUMBER_OPT_NUMBER: &str = "NumberOptNumber";
static COORDS: &str = "NumberOptNumber";
static USER_SPACE_ON_USE: &str = "UserSpaceOnUse";
static OBJECT_BOUNDING_BOX: &str = "ObjectBoundingBox";
static TRANSFORM: &str = "Transform";
static TRANSLATE: &str = "Translate";
static MATRIX: &str = "Matrix";
static SCALE: &str = "Scale";
static ROTATE: &str = "Rotate";
static SKEW_X: &str = "SkewX";
static SKEW_Y: &str = "SkewY";
static TX: &str = "tx";
static TY: &str = "ty";
static SX: &str = "sx";
static SY: &str = "sy";
static CX: &str = "cx";
static CY: &str = "cy";

fn def_angle() -> Opcode {
    Enum::from(ANGLE)
        .fields([
            ("deg", Field::from(Type::Float)),
            ("grad", Field::from(Type::Float)),
            ("rad", Field::from(Type::Float)),
        ])
        .into()
}

fn def_rgb() -> Opcode {
    Data::from(RGB)
        .fields([Type::Byte, Type::Byte, Type::Byte])
        .into()
}

fn def_length() -> Opcode {
    Enum::from(LENGTH)
        .fields([
            (EM, Field::from(Type::Float)),
            (EX, Field::from(Type::Float)),
            (PX, Field::from(Type::Float)),
            (INCH, Field::from(Type::Float)),
            (CM, Field::from(Type::Float)),
            (MM, Field::from(Type::Float)),
            (PT, Field::from(Type::Float)),
            (PC, Field::from(Type::Float)),
            (PERCENT, Field::from(Type::Float)),
        ])
        .into()
}

fn def_iri() -> Opcode {
    Data::from(IRI).fields([Field::from(Type::String)]).into()
}

fn def_func_iri() -> Opcode {
    Enum::from(FUNC_IRI)
        .fields([
            (IRI, Field::from(Type::data(IRI))),
            (PATH, Field::from(Type::String)),
        ])
        .into()
}

fn def_point() -> Opcode {
    Data::from(POINT)
        .fields([Field::from(Type::Float), Field::from(Type::Float)])
        .into()
}

fn def_percentage() -> Opcode {
    Data::from(PERCENT).fields([Type::Float]).into()
}

fn def_paint() -> Opcode {
    Enum::from(PAINT)
        .fields([
            (COLOR, Type::data(RGB)),
            (SERVER, Type::enum_data(FUNC_IRI)),
        ])
        .into()
}

fn def_number_opt_number() -> Opcode {
    Data::from(NUMBER_OPT_NUMBER)
        .fields([
            Field::from(Type::Float),
            Field::from(Type::Float).optional(),
        ])
        .into()
}

fn def_coords() -> Opcode {
    Enum::from(COORDS)
        .fields([
            (USER_SPACE_ON_USE, Type::None),
            (OBJECT_BOUNDING_BOX, Type::None),
        ])
        .into()
}

fn def_transform() -> Opcode {
    Enum::from(TRANSFORM)
        .fields([
            (
                TRANSLATE,
                Data::default().fields([(TX, Type::Float), (TY, Type::Float)]),
            ),
            (
                MATRIX,
                Data::default().fields([
                    Type::Float,
                    Type::Float,
                    Type::Float,
                    Type::Float,
                    Type::Float,
                    Type::Float,
                ]),
            ),
            (
                SCALE,
                Data::default().fields([
                    Field::from((SX, Type::Float)),
                    Field::from((SY, Type::Float)).optional(),
                ]),
            ),
            (
                ROTATE,
                Data::default().fields([
                    Field::from((ANGLE, Type::Float)),
                    Field::from((CX, Type::Float)),
                    Field::from((CY, Type::Float)).optional(),
                ]),
            ),
            (SKEW_X, Data::default().fields([Type::Float])),
            (SKEW_Y, Data::default().fields([Type::Float])),
        ])
        .into()
}

/// Returns vglang syntax opcodes.
pub fn vglang() -> Vec<Opcode> {
    vec![
        def_angle(),
        def_rgb(),
        def_length(),
        def_iri(),
        def_func_iri(),
        def_point(),
        def_percentage(),
        def_paint(),
        def_number_opt_number(),
        def_coords(),
        def_transform(),
    ]
}
