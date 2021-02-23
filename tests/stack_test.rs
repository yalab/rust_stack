use rust_stack::stack::*;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn stack_test() {
    let stack = Stack{};
    stack.push(3);
    assert_eq!(3, stack.pop());
}
