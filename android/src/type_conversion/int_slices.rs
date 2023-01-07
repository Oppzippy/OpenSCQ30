pub fn i16_slice_to_u8_vec(slice: &[i16]) -> anyhow::Result<Vec<u8>> {
    let vec: Vec<u8> = slice
        .iter()
        // TODO log overflows
        .map_while(|x| (*x).try_into().ok())
        .collect();

    anyhow::ensure!(vec.len() == slice.len(), "overflow occurred");

    Ok(vec)
}
