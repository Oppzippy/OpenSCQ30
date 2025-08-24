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
