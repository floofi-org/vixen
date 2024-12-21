use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use vixen_devices::StdinReader;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone)]
pub struct DebuggerStdin {
    buffer: Rc<RefCell<VecDeque<u8>>>,
}

impl DebuggerStdin {
    pub fn new() -> Self {
        let buffer = RefCell::new(VecDeque::new()).into();

        Self {
            buffer,
        }
    }

    pub fn write(&mut self, string: &str) {
        self.buffer
            .borrow_mut()
            .extend(string.as_bytes());
    }
}

impl StdinReader for DebuggerStdin {
    fn read(&self) -> Option<u8> {
        self.buffer.borrow_mut().pop_front()
    }
}
