mod completions;
mod device;
mod list_models;
mod pair;

use clap::{ArgAction, ArgMatches, Command, arg, value_parser};
use macaddr::MacAddr6;
use openscq30_lib::DeviceModel;

pub fn build() -> Command {
    let mac_address_arg = arg!(-a --"mac-address" <MAC_ADDRESS> "Device's mac address")
        .required(true)
        .value_parser(value_parser!(MacAddr6));
    let device_model_arg = arg!(-m --model <MODEL> "Device model")
        .required(true)
        .value_parser(value_parser!(DeviceModel));
    let json_arg = arg!(-j --json "Output as JSON");
    Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Application for managing Soundcore's bluetooth headphones, earbuds, and speakers")
        .max_term_width(100) // Width is 100 by default in clap v5, so remove this when it releases
        .after_help(
"A device must first be paired with openscq30. This is not the same as bluetooth pairing. This refers to informing openscq30 of the mac address and model of the device. See `openscq30 paired-devices`.

Once the device is paired, see `openscq30 device` to interact with device settings."
        )
        .arg(arg!(--"debug-errors" "Displays additional information with errors for debugging purposes"))
        .arg(arg!(-v --verbose "Enables logging"))
        .subcommand_required(true)
        .subcommand(
            Command::new("paired-devices")
                .about("Pair bluetooth devices with openscq30")
                .subcommand_required(true)
                .subcommand(
                    Command::new("add")
                        .about("Pair a device with openscq30")
                        .after_help(
"openscq30 needs to know what model your device is. This is where you provide the necessary information to be able to connect to a device. It should already be paired using bluetooth at this point.

See `openscq30 list-models` for a list of supported device models. This should also be used to find the model id if you only know the name. Soundcore Life Q30 is SoundcoreA3028 for example."
                        )
                        .arg(mac_address_arg.to_owned())
                        .arg(device_model_arg.to_owned())
                        .arg(arg!(--"demo" "Enable demo mode for the device")),
                )
                .subcommand(
                    Command::new("remove")
                        .alias("delete")
                        .about("Remove a paired device")
                        .arg(mac_address_arg.to_owned())
                )
                .subcommand(
                    Command::new("list")
                        .alias("ls")
                        .about("List all currently paired devices")
                        .after_help(
r#"By default, this will output a table with a column for device model, mac address, and if it's a demo or real device.

JSON output (--json) is recommended for scripts. It will instead output an array of objects with the previously mentioned columns as keys instead.

Here is example output with the --json flag:
[
  {
    "macAddress": "00:00:00:00:00:02",
    "model": "SoundcoreA3028",
    "isDemo": true
  },
  {
    "macAddress": "00:00:00:00:00:0D",
    "model": "SoundcoreA3936",
    "isDemo": true
  }
]"#
                        )
                        .arg(json_arg.clone())
                ),
        )
        .subcommand(
            Command::new("device")
                .about("Device settings")
                .arg(mac_address_arg.to_owned())
                .subcommand_required(true)
                .subcommand(
                    Command::new("list-settings")
                        .about("List all currently available settings")
                        .after_help(
r#"All controls exposed by openscq30 have a unique setting id. Those settings can be of varying types, such as a toggle, select, range, etc. Setting categories are only used for showing related settings together to the user. They aren't used otherwise.

This will list all settings that are currently available. Available settings may change depending on the device's state. As an example, some devices only support some features on newer firmware versions.

If `--no-extended-info` is not used, the type of setting as well as its parameters will be output in addition to the setting id.

The format of this command's output is only stable in `--json` mode. The format of the non-json output may change between versions, so `--json` should always be used for scripts.

The equalizer setting has one quirk to be aware of, and that's the fractional digits field. If fractional digits is 1, then 134 would mean 13.4. If it were 2, then 1.34, and so on.

Here is a subset of the JSON output to give an idea of what it looks like. `settings` is an array rather than object with setting ids as keys in order to preserve the order. If that's not relevant, `--no-categories` will have the top level object have setting ids as keys and the setting parameters as its value.

Without --no-categories:
[
  {
    "categoryId": "soundModes",
    "settings": [
      {
        "settingId": "ambientSoundMode",
        "type": "select",
        "setting": {
          "options": [
            "Normal",
            "Transparency",
            "NoiseCanceling"
          ],
          "localizedOptions": [
            "Normal",
            "Transparency",
            "Noise Canceling"
          ]
        }
      }
    ]
  },
  {
    "categoryId": "equalizer",
    "settings": [
      {
        "settingId": "volumeAdjustments",
        "type": "equalizer",
        "setting": {
          "bandHz": [
            100,
            200,
            400,
            800,
            1600,
            3200,
            6400,
            12800
          ],
          "fractionDigits": 1,
          "min": -120,
          "max": 134
        }
      }
    ]
  },
  {
    "categoryId": "deviceInformation",
    "settings": [
      {
        "settingId": "firmwareVersion",
        "type": "information"
      }
    ]
  }
]

With `--no-categories` (only one setting shown as the contents of that object are the same as above, minus the settingId key):
{
  "firmwareVersion": {
    "type": "information"
  }
}
"#
                        )
                        .arg(arg!(--"no-categories" "Don't display category headers"))
                        .arg(arg!(--"no-extended-info" "Don't display setting information in addition to the setting id"))
                        .arg(json_arg.clone())
                )
                .subcommand(
                    Command::new("setting")
                        .about("get/set setting values")
                        .after_help(
r#"`--get` and `--set` may be used multiple times to get/set multiple settings. Operations will be executed in the order of the arguments. A get before a set will print the original value, and a get after a set will print the value it was set to.

Most value types are straight forward. For strings and numbers, enter the value as is. There are some exceptions:

Optional selects: To deselect the value, set it to an empty string: `--set example=`

Modifiable selects: Prefix with '+' to add an item to the list, or '-' to remove an item from the list. The prefix can be escaped with '\'. Examples:
- `--set "customEqualizerProfile=+new profile"` will create a new equalizer profile named "new profile".
- `--set "customEqualizerProfile=-new profile"` will delete an equalizer profile named "new profile".
- `--set "customEqualizerProfile=\+new profile"` will activate an equalizer profile named "+new profile".

Equalizers: All bands must be specified, and numbers should be separated with ','. Examples:
- `--set volumeAdjustments -40,-30,-20,-10,0,1,2,3` when fractional digits is 1 (see `openscq30 device list-settings --help` for info on fractional digits) will assign [-4, -3, -2, -1, 0, 0.1, 0.2, 0.3].
"#
                        )
                        .arg(
                            arg!(-g --get <SETTING_ID> "Gets the value of a setting")
                                .action(ArgAction::Append),
                        )
                        .arg(
                            arg!(-s --set <"SETTING_ID=VALUE"> "Sets the value of a setting.")
                                .action(ArgAction::Append),
                        )
                        .arg(json_arg.clone()),
                )
        )
        .subcommand(
            Command::new("list-models")
                .about("List all supported device models and their names")
                .after_help(
                    "Device models are locale-independent identifiers for each device. This command will list all models as well as their names in English. Used for `openscq30 paired-devices add --model`."
                )
                .arg(json_arg)
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    arg!(-s --shell <SHELL> "Target shell to generate completions for")
                        .required(true)
                        .value_parser(value_parser!(clap_complete::Shell))
                )
        )
}

pub async fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    match matches.subcommand().unwrap() {
        ("paired-devices", matches) => pair::handle(matches).await?,
        ("device", matches) => device::handle(matches).await?,
        ("completions", matches) => completions::handle(matches)?,
        ("list-models", matches) => list_models::handle(matches)?,
        _ => (),
    }
    Ok(())
}
