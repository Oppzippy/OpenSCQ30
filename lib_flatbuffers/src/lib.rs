use flatbuffers::{FlatBufferBuilder, ForwardsUOffset, Vector, WIPOffset};

#[allow(clippy::all, rustdoc::all, warnings)]
pub(crate) mod generated;
pub mod prelude;
mod setting;
pub mod value;

#[derive(thiserror::Error, Debug)]
pub enum FromFlatbufferError {
    #[error(transparent)]
    InvalidFlatbuffer(#[from] flatbuffers::InvalidFlatbuffer),
    #[error("missing required value: {name}")]
    MissingRequiredValue { name: &'static str },
    #[error("invalid union variant for {union_name}: {variant}")]
    InvalidUnionVariant {
        union_name: &'static str,
        variant: u8,
    },
}

pub(crate) fn create_string_vec<'fbb>(
    fbb: &mut FlatBufferBuilder<'fbb>,
    strings: &[impl AsRef<str>],
) -> WIPOffset<Vector<'fbb, ForwardsUOffset<&'fbb str>>> {
    fbb.start_vector::<ForwardsUOffset<&str>>(strings.len());
    for string in strings {
        let offset = fbb.create_string(string.as_ref());
        fbb.push(offset);
    }
    fbb.end_vector::<ForwardsUOffset<&str>>(strings.len())
}
