use rust_stack::stack::*;

#[test]
fn stack_test() {
    let mut stack = Stack::new();
    assert_eq!(1, stack.push(1));
    assert_eq!(3, stack.push(3));
    assert_eq!(3, stack.pop().unwrap());
    assert_eq!(1, stack.pop().unwrap());
}
