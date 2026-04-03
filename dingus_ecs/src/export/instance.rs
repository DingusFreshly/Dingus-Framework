use crate::export::primitive::{DingusTypeHint, RawGetFn, RawMethodFn, RawSetFn};

pub struct PropertyDef {
    pub name: &'static str,
    pub type_hint: DingusTypeHint,
    pub get: RawGetFn,
    pub set:   Option<RawSetFn>,
}

pub struct MethodDef {
    pub name:    &'static str,
    pub handler: RawMethodFn,
}

pub struct InstanceDef {
    pub class_name:     &'static str,
    pub properties:     &'static [PropertyDef],
    pub methods:        &'static [MethodDef],
    ///Ununplemented for now
    pub base_classes:   &'static [&'static str],
}
impl InstanceDef {
    pub const UNREGISTERED : InstanceDef = InstanceDef {
        class_name: "UNREGISTERED INSTANCE",
        properties: &[],
        methods: &[],
        base_classes: &[],
    };
}
//Eventually i will generate these using
pub enum PropertyTag {
    Gui,
    Physics,
}