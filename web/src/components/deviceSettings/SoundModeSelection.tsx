import { Stack } from "@mui/material";
import React, { useCallback } from "react";
import { AmbientSoundModeSelection } from "../soundMode/AmbientSoundModeSelection";
import { NoiseCancelingModeSelection } from "../soundMode/NoiseCancelingModeSelection";
import { SoundModes } from "../../libTypes/DeviceState";

interface Props {
  soundModes: SoundModes;
  setSoundModes: (soundModes: SoundModes) => void;
}

export const SoundModeSelection = React.memo(function ({
  soundModes,
  setSoundModes,
}: Props) {
  const setAmbientSoundMode = useCallback(
    (ambientSoundMode: SoundModes["ambientSoundMode"]) => {
      setSoundModes({
        ...soundModes,
        ambientSoundMode,
      });
    },
    [setSoundModes, soundModes],
  );
  const setNoiseCancelingMode = useCallback(
    (noiseCancelingMode: SoundModes["noiseCancelingMode"]) => {
      setSoundModes({
        ...soundModes,
        noiseCancelingMode,
      });
    },
    [setSoundModes, soundModes],
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
