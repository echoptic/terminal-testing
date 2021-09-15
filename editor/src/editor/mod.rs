mod history;

use std::{
    fs::{self, File},
    io::{self, Write},
};

use ropey::Rope;

use crate::operation::{Operation, Operation::*};

use self::history::History;

pub struct Editor {
    path: String,
    pub text: Rope,
    history: History,
}

impl Editor {
    pub fn new(path: &str) -> Self {
        let path = path.to_string();
        let file = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => String::new(),
        };
        let text = Rope::from_str(&file);
        Self {
            text,
            path,
            history: History::new(),
        }
    }

    fn write(&self) -> io::Result<()> {
        let mut file = File::create(&self.path)?;
        file.write_all(self.text.to_string().as_bytes())?;

        Ok(())
    }

    fn undo(&mut self) {
        if let Some(operation) = self.history.undo() {
            self.run_operation(operation)
        }
    }

    fn redo(&mut self) {
        if let Some(operation) = self.history.redo() {
            self.run_operation(operation)
        }
    }

    pub fn run_operation(&mut self, operation: Operation) {
        match operation {
            Save => {
                let _ = self.write();
            }
            Undo => self.undo(),
            Redo => self.redo(),
            Insert(idx, ref text) => {
                self.text.insert(idx, &text);
            }
            Remove(idx, ref text) => {
                self.text.remove(idx..idx + text.len());
            }
        }

        match operation {
            Save | Undo | Redo => return,
            _ => self.history.store_operation(operation),
        }
    }
}
