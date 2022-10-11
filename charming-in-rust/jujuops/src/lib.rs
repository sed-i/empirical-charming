pub fn calc(n: u64) -> u64 {
	2 * n
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        assert_eq!(calc(0), 0);
        assert_eq!(calc(1), 2);
        assert_eq!(calc(2), 4);
    }
}
