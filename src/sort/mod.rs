/**
 * Quicksorts a numerical array.
 *
 * This algorithm does an inplace sort without
 * additional memory usage.
 */
pub fn mergesort(array: &mut [i32]) -> &mut [i32] {
    let _ = &array.sort();
    return array;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mergesort_empty() {
        assert_eq!(mergesort(&mut []), &mut []);
    }

    #[test]
    fn test_mergesort_with_reversed_order() {
        assert_eq!(mergesort(&mut [5, 4, 3, 2, 1]), &mut [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_mergesort_with_random_order() {
        assert_eq!(mergesort(&mut [3, 4, 2, 5, 1]), &mut [1, 2, 3, 4, 5]);
    }
}
