use flatbuffers::{FlatBufferBuilder, WIPOffset};

use crate::FromFlatbufferError;

pub trait AsFlatbufferExt<'a> {
    type Output;
    fn serialize_to_flatbuffer(
        &'a self,
        fbb: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<Self::Output>;

    fn as_flatbuffer(&'a self) -> Vec<u8> {
        let mut fbb = FlatBufferBuilder::new();
        let root = self.serialize_to_flatbuffer(&mut fbb);
        fbb.finish_minimal(root);
        fbb.finished_data().to_vec()
    }
}

pub trait FromFlatbufferExt
where
    Self: Sized,
{
    fn from_flatbuffer(buffer: &[u8]) -> Result<Self, FromFlatbufferError>;
}
