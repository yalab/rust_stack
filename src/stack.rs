use std::fmt::{self, Formatter, Display};

pub struct Stack{}

impl Stack {
    pub fn push(&self, n: i32) {
    }

    pub fn pop(&self) -> i32 {
        3
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Struct")
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

