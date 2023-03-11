use std::slice;

pub fn i8_slice_to_u8_slice(signed_ints: &[i8]) -> &[u8] {
    unsafe { slice::from_raw_parts(signed_ints.as_ptr() as *const u8, signed_ints.len()) }
}

pub fn u8_slice_to_i8_slice(signed_ints: &[u8]) -> &[i8] {
    unsafe { slice::from_raw_parts(signed_ints.as_ptr() as *const i8, signed_ints.len()) }
}

#[cfg(test)]
mod tests {
    use crate::type_conversion::u8_slice_to_i8_slice;

    use super::i8_slice_to_u8_slice;

    #[test]
    fn i8_to_u8_doesnt_change_underlying_data() {
        let i8s = vec![0, 127, -1, -128];
        let u8s = i8_slice_to_u8_slice(&i8s);
        assert_eq!(vec![0, 127, 255, 128], u8s);
    }

    #[test]
    fn i8_to_u8_doesnt_change_length() {
        let i8s = vec![0, -128];
        assert_eq!(i8s.len(), i8_slice_to_u8_slice(&i8s).len());
    }

    #[test]
    fn i8_to_u8_keeps_empty_slices_empty() {
        let i8s = vec![];
        assert_eq!(true, i8_slice_to_u8_slice(&i8s).is_empty());
    }

    #[test]
    fn u8_to_i8_doesnt_change_underlying_data() {
        let i8s = vec![0, 127, 255, 128];
        let u8s = u8_slice_to_i8_slice(&i8s);
        assert_eq!(vec![0, 127, -1, -128], u8s);
    }

    #[test]
    fn u8_to_i8_doesnt_change_length() {
        let i8s = vec![0, 255];
        assert_eq!(i8s.len(), u8_slice_to_i8_slice(&i8s).len());
    }

    #[test]
    fn u8_to_i8_keeps_empty_slices_empty() {
        let i8s = vec![];
        assert_eq!(true, u8_slice_to_i8_slice(&i8s).is_empty());
    }
}
