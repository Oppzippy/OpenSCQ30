import { Stack } from "@mui/material";
import React, { useCallback } from "react";
import { AmbientSoundModeSelection } from "../soundMode/AmbientSoundModeSelection";
import { NoiseCancelingModeSelection } from "../soundMode/NoiseCancelingModeSelection";
import { SoundModesState } from "../../bluetooth/SoundcoreDeviceState";
import {
  AmbientSoundMode,
  NoiseCancelingMode,
} from "../../../wasm/pkg/openscq30_web_wasm";

interface Props {
  soundModes: SoundModesState;
  setSoundModes: (soundModes: SoundModesState) => void;
}

export const SoundModeSelection = React.memo(function ({
  soundModes,
  setSoundModes,
}: Props) {
  const setAmbientSoundMode = useCallback(
    (ambientSoundMode: AmbientSoundMode) => {
      setSoundModes({
        ambientSoundMode,
        noiseCancelingMode: soundModes.noiseCancelingMode,
      });
    },
    [setSoundModes, soundModes.noiseCancelingMode],
  );
  const setNoiseCancelingMode = useCallback(
    (noiseCancelingMode: NoiseCancelingMode) => {
      setSoundModes({
        ambientSoundMode: soundModes.ambientSoundMode,
        noiseCancelingMode,
      });
    },
    [setSoundModes, soundModes.ambientSoundMode],
  );

  return (
    <Stack spacing="2">
      <AmbientSoundModeSelection
        value={soundModes.ambientSoundMode}
        onValueChanged={setAmbientSoundMode}
      />
      <NoiseCancelingModeSelection
        value={soundModes.noiseCancelingMode}
        onValueChanged={setNoiseCancelingMode}
      />
    </Stack>
  );
});
