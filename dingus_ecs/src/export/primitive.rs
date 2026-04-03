use crate::{Entity, World};
use crate::export::export_error::ScriptError;

/// All the properties that can be exported into the editor.
#[derive(Debug, Clone, PartialEq)]
pub enum DingusPrimitive {
    Int     (i64),
    Float   (f64),
    Unsigned(u64),
    Bool    (bool),
    String  (String),
}
pub enum DingusTypeHint {
    Int,
    Float,
    Unsigned,
    Bool,
    String
}

impl DingusPrimitive {
    pub fn  type_hint(&self) -> DingusTypeHint {
        match self { 
            DingusPrimitive::Int(..)     => DingusTypeHint::Int,
            DingusPrimitive::Float(..)   => DingusTypeHint::Float,
            DingusPrimitive::Unsigned(..)=> DingusTypeHint::Unsigned,
            DingusPrimitive::Bool(..)    => DingusTypeHint::Bool,
            DingusPrimitive::String(..)  => DingusTypeHint::String
        }
    }
}

pub type RawGetFn    = fn(&World, Entity) -> Result<DingusPrimitive, ScriptError>;
pub type RawSetFn    = fn(&mut World, Entity, DingusPrimitive) -> Result<(), ScriptError>;
pub type RawMethodFn = fn(&mut World, Entity, &[DingusPrimitive]) -> Result<&'static [DingusPrimitive], ScriptError>;


pub fn to_primitive<T: Into<DingusPrimitive>>(value: T) -> DingusPrimitive {
    value.into()
}
pub fn from_primitive<T: From<DingusPrimitive>>(value: DingusPrimitive) -> T { value.into()}
impl<T> From<&T> for DingusPrimitive
where
    T: Clone + Into<DingusPrimitive>,
{
    fn from(value: &T) -> Self {
        value.clone().into()
    }
}

impl<T> From<*mut T> for DingusPrimitive {
    fn from(value: *mut T) -> Self {
        value.into()
    }
}