// modulo op with negative number
// https://stackoverflow.com/questions/11720656/modulo-operation-with-negative-numbers
pub fn reminder(a: i32, m: i32) -> i32 {
    ((a % m) + m) % m
}
