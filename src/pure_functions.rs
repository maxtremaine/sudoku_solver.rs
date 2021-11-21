pub fn get_missing_digits(digits: Vec<u8>) -> Vec<u8> {
    return (1..10)
        .filter(|digit| !digits.contains(digit))
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_missing_digits() {
        let test_list: Vec<u8> = (4..10).collect();
        let test_output: Vec<u8> = vec![1, 2, 3];
        assert_eq!(test_output, get_missing_digits(test_list));
    }
}