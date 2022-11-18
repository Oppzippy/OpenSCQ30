pub fn calculate_checksum(checksum_data: &[u8]) -> u8 {
    checksum_data
        .iter()
        .fold(0 as u8, |acc, curr| acc.wrapping_add(*curr))
}
