pub fn f_to_c(temp: f32) -> f32 {
    (temp - 32.0) / 9.0 * 5.0
}

#[test]
fn test_f_to_c() {
    assert_eq!(f_to_c(32.0), 0.0);
    assert_eq!(f_to_c(212.0), 100.0);
}

pub fn c_to_f(temp: f32) -> f32 {
    temp * 1.8 + 32.0
}

#[test]
fn test_c_to_f() {
    assert_eq!(c_to_f(0.0), 32.0);
    assert_eq!(c_to_f(100.0), 212.0);
}
