#[cfg(test)]
pub mod test {
    use crate::*;

    #[test]
    fn test_number_of_loops_works_correctly() {
        let result = number_of_loops_in_number("8809");
        assert_eq!(result, 6);
    }
    #[test]
    fn test_number_of_loops_works_correctly2() {
        let result = number_of_loops_in_number("6666");
        assert_eq!(result, 4);
    }
    #[test]
    fn test_number_of_loops_works_correctly3() {
        let result = number_of_loops_in_number("2581");
        assert_eq!(result, 2);
    }
    #[test]
    fn test_number_of_loops_no_loops() {
        let under_test = number_of_loops_in_number("7111");
        assert_eq!(under_test, 0);
    }
}
