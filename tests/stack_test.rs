use rust_stack::stack::*;

#[test]
fn stack_test() {
    let mut stack = Stack::new();
    assert_eq!(1, stack.push(1));
    stack.push(3);
    match stack.pop() {
        Some(n) => assert_eq!(3, n),
        None    => panic!()
    }
    match stack.pop() {
        Some(n) => assert_eq!(1, n),
        None    => panic!()
    }
}

