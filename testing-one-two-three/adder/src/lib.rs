pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_is_a_unit_test_in_the_file_with_the_code() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
