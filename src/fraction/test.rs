use super::*;

#[test]
fn test_add() {
    struct Case(Fraction, Fraction, Fraction);
    let cases = vec![
        Case(Fraction(1, 2), Fraction(1, 3), Fraction(5, 6)),
        Case(Fraction(2, 3), Fraction(1, 4), Fraction(11, 12)),
        Case(Fraction(5, 6), Fraction(4, 15), Fraction(11, 10)),
    ];

    for Case(lhs, rhs, result) in cases {
        assert_eq!(result, lhs + rhs)
    }
}

#[test]
fn test_mul() {
    struct Case(Fraction, Fraction, Fraction);
    let cases = vec![
        Case(Fraction(1, 2), Fraction(1, 3), Fraction(1, 6)),
        Case(Fraction(2, 3), Fraction(1, 5), Fraction(2, 15)),
        Case(Fraction(5, 6), Fraction(4, 15), Fraction(2, 9)),
    ];

    for Case(lhs, rhs, result) in cases {
        assert_eq!(result, lhs * rhs)
    }
}

#[test]
fn test_div() {
    // TODO
}