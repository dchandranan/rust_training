use p22::calc;

#[test]
fn test_temperature_conversions() {
    assert_eq!(calc::celsius2farenheit(0), 32);
    assert_eq!(calc::farenheit2celsius(32), 0);
}

#[test]
fn test_fibonacci_functions() {
    let fib_numbers = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    for (n, &expected) in fib_numbers.iter().enumerate() {
        assert_eq!(calc::fibonacci_loop(n as u32), expected);
        assert_eq!(calc::fibonacci_rec(n as u32), expected);
    }
}
