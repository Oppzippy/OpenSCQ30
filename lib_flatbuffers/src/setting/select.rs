use std::borrow::Cow;

use openscq30_lib::api::settings;

use crate::{create_string_vec, generated, prelude::AsFlatbufferExt};

impl From<generated::setting::Select<'_>> for settings::Select {
    fn from(select: generated::setting::Select) -> Self {
        settings::Select {
            options: select
                .options()
                .into_iter()
                .map(ToOwned::to_owned)
                .map(Cow::Owned)
                .collect(),
            localized_options: select
                .localized_options()
                .into_iter()
                .map(ToOwned::to_owned)
                .collect(),
        }
    }
}

impl<'a> AsFlatbufferExt<'a> for settings::Select {
    type Output = generated::setting::Select<'a>;

    fn serialize_to_flatbuffer(
        &'a self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> flatbuffers::WIPOffset<Self::Output> {
        let options = create_string_vec(fbb, &self.options);
        let localized_options = create_string_vec(fbb, &self.localized_options);
        generated::setting::Select::create(
            fbb,
            &generated::setting::SelectArgs {
                options: Some(options),
                localized_options: Some(localized_options),
            },
        )
    }
}
