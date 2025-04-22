use openscq30_lib::api::settings;

use crate::{generated, prelude::AsFlatbufferExt};

impl From<generated::setting::Equalizer<'_>> for settings::Equalizer {
    fn from(equalizer: generated::setting::Equalizer) -> Self {
        settings::Equalizer {
            band_hz: equalizer.band_hz().into_iter().collect(),
            fraction_digits: equalizer.fraction_digits(),
            min: equalizer.min(),
            max: equalizer.max(),
        }
    }
}

impl<'a> AsFlatbufferExt<'a> for settings::Equalizer {
    type Output = generated::setting::Equalizer<'a>;

    fn serialize_to_flatbuffer(
        &'a self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<Self::Output> {
        let band_hz = fbb.create_vector(self.band_hz.as_ref());
        generated::setting::Equalizer::create(
            fbb,
            &generated::setting::EqualizerArgs {
                band_hz: Some(band_hz),
                fraction_digits: self.fraction_digits,
                min: self.min,
                max: self.max,
            },
        )
    }
}
