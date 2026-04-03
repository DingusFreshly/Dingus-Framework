pub type DenseIndex = u32;

pub enum RegisterType {
    Component,
    Archetype,
    Resource,
}

pub trait RegisteredLocation {
    const DENSE_INDEX: DenseIndex;
    const NAME : &'static str;

    const REGISTER_TYPE: RegisterType;

}
