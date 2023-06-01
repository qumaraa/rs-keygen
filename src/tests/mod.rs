#[cfg(test)]
mod tests {
    /// Tests
    use super::super::KeyGen;
    #[test]
    /// default keygen test
    fn test_keygen_def() {
        let mut kg = KeyGen::new().lowercase(true).length(11);
        let gen_key = kg.gen_one().unwrap();
        assert_eq!(gen_key.len(), 11);
        assert_eq!(kg.length, 11);
        assert_eq!(kg.symbols, false);
        assert_eq!(kg.numbers, false);
        assert_eq!(kg.space, false);
        assert_eq!(kg.lowercase, true);
        assert_eq!(kg.uppercase, false);
    }

    #[test]
    fn test_keygen() {
        let mut kg = KeyGen::new().length(10).symbols(true).numbers(true);
        let gen_key = kg.gen_one().unwrap();
        assert_eq!(gen_key.len(), 10);
        assert_eq!(kg.length, 10);
        assert_eq!(kg.symbols, true);
        assert_eq!(kg.numbers, true);
    }
}