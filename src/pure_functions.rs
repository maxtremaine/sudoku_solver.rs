pub fn get_missing_digits(digits: Vec<u8>) -> Vec<u8> {
    return (1..10)
        .filter(|digit| !digits.contains(digit))
        .collect::<Vec<u8>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_missing_digits() {
        let test_list = (4..10).collect::<Vec<u8>>();
        let test_output: Vec<u8> = vec![1, 2, 3];
        assert_eq!(test_output, get_missing_digits(test_list));
    }
}