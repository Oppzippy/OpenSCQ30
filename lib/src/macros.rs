/// Implements `From` for error types that wrap a source error along with the
/// source code location that the error occurred.
///
/// # Example
/// ```ignore
/// use crate::macros::impl_from_source_error_with_location;
///
/// enum OuterError {
///     InnerError {
///         source: InnerError,
///         location: &'static std::panic::Location<'static>,
///     },
/// }
///
/// struct InnerError;
///
/// impl_from_source_error_with_location!(OuterError::InnerError(InnerError));
///
/// fn fail() -> Result<(), OuterError> {
///     Err(OuterError::from(InnerError))
/// }
///
/// assert!(matches!(fail().unwrap_err(), OuterError::InnerError { .. }))
/// ```
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

/// Generates an enum that is a subset of an existing enum, along with a TryFrom implementation for superset to subset,
/// and a From implementation for subset to superset.
///
/// # Examples
///
/// ```ignore
/// use crate::macros::enum_subset;
///
/// #[derive(Debug, PartialEq)]
/// enum Superset {
///     A,
///     B,
/// }
///
/// enum_subset! {
///     Superset,
///     #[derive(Debug, PartialEq)]
///     enum Subset {
///         A,
///     }
/// };
///
/// assert_eq!(Superset::from(Subset::A), Superset::A);
/// assert_eq!(Subset::try_from(Superset::A), Superset::A);
/// assert!(Subset::try_from(Superset::B).is_err());
/// ```
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
