/// Converts Celsius to Fahrenheit
/// 
/// # Examples
/// ```
/// use p22::calc::celsius2farenheit;
/// assert_eq!(celsius2farenheit(0), 32);
/// assert_eq!(celsius2farenheit(100), 212);
/// ```
pub fn celsius2farenheit(celsius: i32) -> i32 {
    (celsius * 9/5) + 32
}

/// Converts Fahrenheit to Celsius
/// 
/// # Examples
/// ```
/// use p22::calc::farenheit2celsius;
/// assert_eq!(farenheit2celsius(32), 0);
/// assert_eq!(farenheit2celsius(212), 100);
/// ```
pub fn farenheit2celsius(farenheit: i32) -> i32 {
    (farenheit - 32) * 5/9
}

/// Calculates Fibonacci number using loop
/// 
/// # Examples
/// ```
/// use p22::calc::fibonacci_loop;
/// assert_eq!(fibonacci_loop(0), 0);
/// assert_eq!(fibonacci_loop(1), 1);
/// assert_eq!(fibonacci_loop(10), 55);
/// ```
pub fn fibonacci_loop(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut a = 0;
    let mut b = 1;
    //let mut c = 0;
    for _ in 1..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}

/// Calculates Fibonacci number using recursion
/// 
/// # Examples
/// ```
/// use p22::calc::fibonacci_rec;
/// assert_eq!(fibonacci_rec(0), 0);
/// assert_eq!(fibonacci_rec(1), 1);
/// assert_eq!(fibonacci_rec(10), 55);
/// ```
pub fn fibonacci_rec(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_rec(n-1) + fibonacci_rec(n-2)
    }
}