use crate::export::DingusTypeHint;

pub enum ScriptError {
    ValueNotFound{
        name: String
    },
    UnexpectedType{
        expected: DingusTypeHint,
        got: DingusTypeHint
    }

}

