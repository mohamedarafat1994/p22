use p22::calculator;

#[test]
fn test_celsius2farenheit() {
    assert_eq!(calculator::celsius2farenheit(30), 86);
}

#[test]
fn test_farenheit2celsius() {
    assert_eq!(calculator::farenheit2celsius(86), 30);
}

#[test]
fn test_fibonacci_loop() {
    assert_eq!(calculator::fibonacci_loop(3), 2);
}

#[test]
fn test_fibonacci_rec() {
    assert_eq!(calculator::fibonacci_rec(3), 2);
}
