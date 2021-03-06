// modulo op with negative number
// https://stackoverflow.com/questions/11720656/modulo-operation-with-negative-numbers
pub fn reminder(a: i32, m: i32) -> i32 {
    ((a % m) + m) % m
}

#[test]
fn t_reminder() {
    assert_eq!(reminder(-7, 26), 19);
}
