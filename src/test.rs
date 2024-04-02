pub fn answer(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

#[test]
fn check_answer_validity() {
    assert_eq!(answer(40, 2), 42);
}
