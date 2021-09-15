use self::Operation::*;

#[derive(Debug, Clone)]
pub enum Operation {
    Save,

    Undo,
    Redo,

    Insert(usize, String),
    Remove(usize, String),
}

impl Operation {
    pub fn invert(&self) -> Self {
        match self {
            Save | Undo | Redo => unreachable!(),
            Insert(idx, text) => Remove(*idx, text.into()),
            Remove(idx, text) => Insert(*idx, text.into()),
        }
    }
}
