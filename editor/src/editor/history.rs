use crate::operation::Operation;

pub struct History {
    undo: Vec<Operation>,
    redo: Vec<Operation>,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    pub fn undo(&mut self) -> Option<Operation> {
        if self.undo.is_empty() {
            return None;
        }

        let operation = self.undo.pop();
        self.redo.push(operation.clone().unwrap().invert());

        operation
    }

    pub fn redo(&mut self) -> Option<Operation> {
        if self.redo.is_empty() {
            return None;
        }

        let operation = self.redo.pop();
        self.undo.push(operation.clone().unwrap().invert());

        operation
    }

    pub fn store_operation(&mut self, operation: Operation) {
        self.undo.push(operation.invert())
    }
}
