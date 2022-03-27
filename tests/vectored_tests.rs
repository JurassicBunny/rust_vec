use vectored::{Acceleration, Vectored};

#[test]
fn test_norm() {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let second_vec = Acceleration::new(2.0, 3.0, 4.0);

    assert_eq!(vector.norm(), 14.0_f64.sqrt());
    assert_eq!(second_vec.norm(), 29.0_f64.sqrt());
}

#[test]
fn test_sqr_norm() {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let second_vec = Acceleration::new(2.0, 3.0, 4.0);

    assert_eq!(vector.sqr_norm(), 14.0);
    assert_eq!(second_vec.sqr_norm(), 29.0);
}

#[test]
fn test_normalize() {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let second_vec = Acceleration::new(2.0, 3.0, 4.0);

    assert_eq!(vector.normalize().norm(), 1.0);
    assert_eq!(second_vec.normalize().norm(), 1.0);
}

#[test]
fn test_addition() {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let second_vec = Acceleration::new(2.0, 3.0, 4.0);

    let result = Acceleration::new(3.0, 5.0, 7.0);

    assert_eq!(vector + second_vec, result);
}

#[test]
fn test_subtraction() {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let second_vec = Acceleration::new(2.0, 3.0, 4.0);

    let result = Acceleration::new(-1.0, -1.0, -1.0);

    assert_eq!(vector - second_vec, result);
}

#[test]
fn test_scalar() {
    let vector = Acceleration::new(1.0, 2.0, 3.0);
    let scalar = 2.0;

    let result = Acceleration::new(2.0, 4.0, 6.0);

    assert_eq!(vector * scalar, result);
    assert_eq!(scalar * vector, result);
}
