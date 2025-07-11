pub fn num_digits(mut n: usize) -> usize {
    if n == 0 {
        1
    } else {
        let mut digits = 0;
        while n > 0 {
            n /= 10;
            digits += 1;
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_digits() {
        assert_eq!(num_digits(0), 1);
        assert_eq!(num_digits(5), 1);
        assert_eq!(num_digits(10), 2);
        assert_eq!(num_digits(123), 3);
        assert_eq!(num_digits(9999), 4);
        assert_eq!(num_digits(1000000), 7);
    }
}
