mod test_literal {
    use paste::paste;

    const LEN: &str = paste! {
        [! "abc":len !]
    };

    #[test]
    fn test_literal() {
        assert_eq!(LEN, "3");
    }
}
