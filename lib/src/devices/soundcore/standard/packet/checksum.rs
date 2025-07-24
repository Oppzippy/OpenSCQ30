pub fn calculate_checksum<'a>(checksum_data: impl IntoIterator<Item = &'a u8>) -> u8 {
    checksum_data
        .into_iter()
        .fold(0_u8, |acc, curr| acc.wrapping_add(*curr))
}

#[cfg(test)]
mod tests {
    use super::calculate_checksum;

    #[test]
    fn checksum_is_correct_with_no_data() {
        let checksum = calculate_checksum(&[]);
        assert_eq!(0, checksum);
    }

    #[test]
    fn checksum_is_correct_without_wrapping() {
        let checksum = calculate_checksum(&[1, 2]);
        assert_eq!(3, checksum);
    }

    #[test]
    fn checksum_is_correct_with_wrapping() {
        let checksum = calculate_checksum(&[0xff, 0x02]);
        assert_eq!(0x01, checksum);
    }
}
