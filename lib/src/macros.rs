macro_rules! impl_as_ref_for_field {
    (struct $struct_type:ty {$($field_name:ident: $field_type:ty,)+}) => {
        $(
            $crate::macros::impl_as_ref_for_field!($struct_type as $field_name: $field_type);
        )+
    };

    ($struct_type:ty as $field_name:ident: $field_type:ty) => {
        impl AsRef<$field_type> for $struct_type {
            fn as_ref(&self) -> &$field_type {
                &self.$field_name
            }
        }
        impl AsMut<$field_type> for $struct_type {
            fn as_mut(&mut self) -> &mut $field_type {
                &mut self.$field_name
            }
        }
    };
}
pub(crate) use impl_as_ref_for_field;
