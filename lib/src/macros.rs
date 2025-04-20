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

macro_rules! impl_from_source_error_with_location {
    ($target_error:ident::$target_variant:ident($source_error:ty)) => {
        impl From<$source_error> for $target_error {
            #[track_caller]
            fn from(source: $source_error) -> $target_error {
                $target_error::$target_variant {
                    source,
                    location: Location::caller(),
                }
            }
        }
    };
}
pub(crate) use impl_from_source_error_with_location;
