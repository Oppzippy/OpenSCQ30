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

macro_rules! enum_subset {
    (
        $superset_enum:ident,
        // pass through derives and such
        $(#[$($attribute:tt)*])*
        $vis:vis enum $subset_enum:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$($attribute)*])*
        $vis enum $subset_enum {
            $($variant,)*
        }

        impl From<$subset_enum> for $superset_enum {
            fn from(value: $subset_enum) -> Self {
                match value {
                    $($subset_enum::$variant => Self::$variant,)*
                }
            }
        }

        impl TryFrom<$superset_enum> for $subset_enum {
            type Error = ();

            fn try_from(value: $superset_enum) -> Result<Self, Self::Error> {
                match value {
                    $($superset_enum::$variant => Ok(Self::$variant),)*
                    _ => Err(())
                }
            }
        }
    };
}
pub(crate) use enum_subset;
