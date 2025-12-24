use clap::ArgMatches;
use openscq30_i18n::Translate;
use openscq30_lib::DeviceModel;
use serde::Serialize;
use strum::VariantArray;
use tabled::{Table, Tabled};

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    let models = {
        let mut models = DeviceModel::VARIANTS.to_vec();
        models.sort_by_key(|model| <&'static str>::from(model));
        models
    };

    if matches.get_flag("json") {
        let json = serde_json::to_string_pretty(
            &models
                .into_iter()
                .map(|model| ModelJsonItem {
                    model,
                    name: model.translate(),
                })
                .collect::<Vec<ModelJsonItem>>(),
        )?;
        println!("{}", json);
    } else {
        let mut table = Table::new(models.into_iter().map(|model| ModelTableItem {
            model,
            name: model.translate(),
        }));
        crate::fmt::apply_tabled_settings(&mut table);
        println!("{table}");
    }

    Ok(())
}

#[derive(Tabled)]
struct ModelTableItem {
    #[tabled(rename = "Model")]
    pub model: DeviceModel,
    #[tabled(rename = "Name")]
    pub name: String,
}

#[derive(Serialize)]
struct ModelJsonItem {
    pub model: DeviceModel,
    pub name: String,
}
