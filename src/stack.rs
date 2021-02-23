use std::fmt::{self, Formatter, Display};

pub struct Stack{
    xs: Vec<i32>
}

impl Stack {
    pub fn new() -> Stack {
        let xs = Vec::new();
        Stack{xs: xs}
    }

    pub fn push(&mut self, n: i32) {
        self.xs.push(n)
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.xs.pop()
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Struct")
    }
}
