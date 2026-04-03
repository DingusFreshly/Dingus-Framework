use crate::internal::*;


macro_rules! dingus_primitive {
    // integers
    (int: $($t:ty),*) => {
        $(
            impl From<$t> for DingusPrimitive {
                fn from(v: $t) -> Self { DingusPrimitive::Int(v as i64) }
            }
            impl std::convert::TryFrom<DingusPrimitive> for $t {
                type Error = ScriptError;
                fn try_from(v: DingusPrimitive) -> Result<Self, Self::Error> {
                    match v {
                        DingusPrimitive::Int(n) => Ok(n as $t),
                        other => Err(ScriptError::UnexpectedType {
                            got: other.type_hint(),
                            expected: DingusTypeHint::Int
                        }),
                    }
                }
            }
        )*
    };

    // unsigned integers
    (uint: $($t:ty),*) => {
        $(
            impl From<$t> for DingusPrimitive {
                fn from(v: $t) -> Self { DingusPrimitive::Unsigned(v as u64) }
            }
            impl std::convert::TryFrom<DingusPrimitive> for $t {
                type Error = ScriptError;
                fn try_from(v: DingusPrimitive) -> Result<Self, Self::Error> {
                    match v {
                        DingusPrimitive::Unsigned(n) => Ok(n as $t),
                        other => Err(ScriptError::UnexpectedType {
                            got: other.type_hint(),
                            expected: DingusTypeHint::Unsigned
                        }),
                    }
                }
            }
        )*
    };

    // floats
    (float: $($t:ty),*) => {
        $(
            impl From<$t> for DingusPrimitive {
                fn from(v: $t) -> Self { DingusPrimitive::Float(v as f64) }
            }
            impl std::convert::TryFrom<DingusPrimitive> for $t {
                type Error = ScriptError;
                fn try_from(v: DingusPrimitive) -> Result<Self, Self::Error> {
                    match v {
                        DingusPrimitive::Float(f) => Ok(f as $t),
                        other => Err(ScriptError::UnexpectedType {
                            got: other.type_hint(),
                            expected: DingusTypeHint::Float
                        }),
                    }
                }
            }
        )*
    };

    // bool
    (bool) => {
        impl From<bool> for DingusPrimitive { fn from(v: bool) -> Self { DingusPrimitive::Bool(v) } }
        impl std::convert::TryFrom<DingusPrimitive> for bool {
            type Error = ScriptError;
            fn try_from(v: DingusPrimitive) -> Result<Self, Self::Error> {
                match v {
                    DingusPrimitive::Bool(b) => Ok(b),
                    other => Err(ScriptError::UnexpectedType {
                        got: other.type_hint(),
                        expected: DingusTypeHint::Bool
                    }),
                }
            }
        }
    };

}

// Usage:

dingus_primitive!(int: i8, i16, i32, i64);
dingus_primitive!(uint: u8, u16, u32, u64);
dingus_primitive!(float: f32, f64);
dingus_primitive!(bool);

impl From<&'static str> for DingusPrimitive { fn from(v: &'static str) -> Self { DingusPrimitive::String(v.into()) } }

impl From<String> for DingusPrimitive { fn from(v: String) -> Self { DingusPrimitive::String(v) } }
impl TryFrom<DingusPrimitive> for String {
    type Error = ScriptError;
    fn try_from(v: DingusPrimitive) -> Result<Self, Self::Error> {
        match v {
            DingusPrimitive::String(s) => Ok(s.into()),
            other => Err(ScriptError::UnexpectedType {
                got: other.type_hint(),
                expected: DingusTypeHint::String
            }),
        }
    }
}