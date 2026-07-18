#!/usr/bin/env python3
# takes in a list of set eq packets and outputs EqualizerPreset structs
# input should be one packet per line in the format: [1, 2, 3, ...]
# first argument is number of bands, second is optionally a file containing preset names in order

import sys
import textwrap
import pathlib
import re
from dataclasses import dataclass

bands = int(sys.argv[1])
if bands < 1:
    print("bands (arg 1) must be >1")
    exit(1)

preset_names = []
if len(sys.argv) >= 3:
    preset_names_path = pathlib.Path(sys.argv[2])
    content = preset_names_path.read_text()
    preset_names = content.splitlines()


@dataclass
class Preset:
    id: int
    gains: list[int]
    name: str


presets: list[Preset] = []
while line := sys.stdin.readline():
    packet = [
        int(byte_text.strip()) for byte_text in line.strip().strip("[]").split(",")
    ]
    name = ""
    if len(presets) < len(preset_names):
        name = preset_names[len(presets)]
    if packet[5:7] == [2, 129] or packet[5:7] == [2, 131] or packet[5:7] == [3, 134]:
        preset_id = int.from_bytes(bytes(packet[9:11]), byteorder="little")
        gain_bytes = packet[11 : 11 + bands]
        gains = [gain - 120 for gain in gain_bytes]
        presets.append(Preset(preset_id, gains, name))
    elif packet[5:7] == [3, 135]:
        preset_id = int.from_bytes(bytes(packet[9:11]), byteorder="little")
        gain_bytes = packet[13 : 13 + bands]
        gains = [gain - 120 for gain in gain_bytes]
        presets.append(Preset(preset_id, gains, name))

presets.sort(key=lambda preset: preset.id)

pascal_case_to_spine_case = re.compile(r"(?<!^)(?=[A-Z])")
for i, preset in enumerate(presets):
    name_spine_case = pascal_case_to_spine_case.sub("-", preset.name).lower()

    print(
        textwrap.dedent(
            f"""
            EqualizerPreset {{
                name: "{preset.name}",
                localized_name: || fl!("{name_spine_case}"),
                id: {preset.id},
                volume_adjustments: VolumeAdjustments::new({preset.gains}),
            }},
            """
        ).strip("\n")
    )
