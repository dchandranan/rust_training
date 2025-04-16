/// Mutable reference to first element if flag is false, second if true
pub fn f1(tuple: &mut (u32, u32, bool)) -> &mut u32 {
    if tuple.2 { &mut tuple.1 } else { &mut tuple.0 }
}

/// Returns mutable reference to n-th element in slice
pub fn f2(slice: &mut [u32], n: usize) -> &mut u32 {
    &mut slice[n]
}

/// Returns mutable reference to n-th element from end
/// ///
/// # Examples
/// ```
/// use p24::refs::f3;
/// let mut arr = [1, 2, 3, 4, 5];
/// *f3(&mut arr, 1) = 9;
/// assert_eq!(arr, [1, 2, 3, 9, 5]);
/// ```
///
pub fn f3(slice: &mut [u32], n: usize) -> &mut u32 {
    let idx = slice.len() - 1 - n;
    &mut slice[idx]
}

/// Partitions slice into 4 roughly equal parts
///
/// # Examples
/// ```
/// use p24::refs::f4;
/// let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let (a, b, c, d) = f4(&arr);
/// assert_eq!(a, &[1, 2, 3]);
/// assert_eq!(b, &[4, 5, 6]);
/// assert_eq!(c, &[7, 8]);
/// assert_eq!(d, &[9, 10]);
/// ```
pub fn f4(slice: &[u32]) -> (&[u32], &[u32], &[u32], &[u32]) {
    let len = slice.len();
    let chunk_size = len / 4;
    let remainder = len % 4;

    // Calculate cutoffs with remainder distributed
    let end1 = chunk_size + (remainder > 0) as usize;
    let end2 = end1 + chunk_size + (remainder > 1) as usize;
    let end3 = end2 + chunk_size + (remainder > 2) as usize;

    (
        &slice[..end1],
        &slice[end1..end2],
        &slice[end2..end3],
        &slice[end3..],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f1() {
        let mut tuple = (10, 20, false);
        *f1(&mut tuple) = 30;
        assert_eq!(tuple, (30, 20, false));

        let mut tuple = (10, 20, true);
        *f1(&mut tuple) = 30;
        assert_eq!(tuple, (10, 30, true));
    }

    #[test]
    fn test_f2() {
        let mut arr = [1, 2, 3, 4];
        *f2(&mut arr, 2) = 30;
        assert_eq!(arr, [1, 2, 30, 4]);
    }

    #[test]
    fn test_f3() {
        let mut arr = [1, 2, 3, 4];
        *f3(&mut arr, 2) = 30;
        assert_eq!(arr, [1, 30, 3, 4]);
    }

    #[test]
    fn test_f4() {
        // Test even division
        let arr = [1, 2, 3, 4, 5, 6, 7, 8];
        let (a, b, c, d) = f4(&arr);
        assert_eq!(a, &[1, 2]);
        assert_eq!(b, &[3, 4]);
        assert_eq!(c, &[5, 6]);
        assert_eq!(d, &[7, 8]);

        // Test uneven division
        let arr = [1, 2, 3, 4, 5];
        let (a, b, c, d) = f4(&arr);
        assert_eq!(a, &[1, 2]);
        assert_eq!(b, &[3]);
        assert_eq!(c, &[4]);
        assert_eq!(d, &[5]);
    }

    #[test]
    fn test_f4_edge_cases() {
        // Empty slice
        let (a, b, c, d) = f4(&[]);
        assert_eq!(a, &[]);
        assert_eq!(b, &[]);
        assert_eq!(c, &[]);
        assert_eq!(d, &[]);

        // Single element
        let (a, b, c, d) = f4(&[42]);
        assert_eq!(a, &[42]);
        assert_eq!(b, &[]);
        assert_eq!(c, &[]);
        assert_eq!(d, &[]);

        // Three elements
        let (a, b, c, d) = f4(&[1, 2, 3]);
        assert_eq!(a, &[1]);
        assert_eq!(b, &[2]);
        assert_eq!(c, &[3]);
        assert_eq!(d, &[]);
    }
}
