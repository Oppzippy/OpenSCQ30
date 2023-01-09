use std::slice;

pub fn i8_slice_to_u8_slice(signed_ints: &[i8]) -> &[u8] {
    unsafe { slice::from_raw_parts(signed_ints.as_ptr() as *const u8, signed_ints.len()) }
}

pub fn u8_slice_to_i8_slice(signed_ints: &[u8]) -> &[i8] {
    unsafe { slice::from_raw_parts(signed_ints.as_ptr() as *const i8, signed_ints.len()) }
}
