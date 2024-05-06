use super::*;

#[test]
fn test_zero_celsius() {
    assert_eq!(cels_to_faren(0.), 32.);
}

#[test]
fn test_below_zero_celsius() {
    assert_eq!(cels_to_faren(-10.), 14.);
}

#[test]
fn test_float_celsius() {
    // Sometimes just IEEE754 sucks :(
    assert_eq!(cels_to_faren(2.35), 36.230000000000004);
}

#[test]
fn test_below_zero_faren() {
    assert_eq!(cels_to_faren(-150.), -238.);
}
