import { Input, Slider, Typography } from "@mui/material";
import Grid2 from "@mui/material/Unstable_Grid2";
import React from "react";
import { useTranslation } from "react-i18next";
import { EqualizerHelper } from "../../../wasm/pkg/openscq30_web_wasm";

interface Props {
  hz: number;
  disabled?: boolean;
  value: number;
  onValueChange: (index: number, newValue: number) => void;
  index: number;
}

export const VolumeSlider = React.memo(function (props: Props) {
  const { t } = useTranslation();
  const labelId = `${props.hz}-hz-label`;
  const step = 0.1;
  const minVolume = EqualizerHelper.MIN_VOLUME;
  const maxVolume = EqualizerHelper.MAX_VOLUME;
  const displayValue = Math.round(props.value * 10) / 10;
  return (
    <>
      {/* make sure Hz doesn't go on to a second line */}
      <Grid2 xs={3} sm={2}>
        <div id={labelId}>
          <Typography>
            {props.hz >= 1000
              ? t("equalizer.khz", {
                  defaultValue: "{{ hz }} kHz",
                  replace: { hz: props.hz / 1000 },
                })
              : t("equalizer.hz", {
                  defaultValue: "{{ hz }} Hz",
                  replace: { hz: props.hz },
                })}
          </Typography>
        </div>
      </Grid2>
      <Grid2 xs={7} sm={8}>
        <Slider
          disabled={props.disabled}
          value={displayValue}
          min={minVolume}
          max={maxVolume}
          step={step}
          valueLabelDisplay="auto"
          aria-labelledby={labelId}
          marks={[
            { value: minVolume, label: `${minVolume} dB` },
            { value: maxVolume, label: `${maxVolume} dB` },
          ]}
          onChange={(_, value) => {
            if (typeof value == "number") {
              props.onValueChange(props.index, value);
            } else {
              throw Error(
                `Expected single number, got number array: ${value.toString()}`,
              );
            }
          }}
        />
      </Grid2>
      <Grid2 xs={2}>
        <Input
          disabled={props.disabled}
          value={displayValue}
          onChange={(event) =>
            props.onValueChange(props.index, Number(event.target.value))
          }
          size="small"
          inputProps={{
            type: "number",
            min: minVolume,
            max: maxVolume,
            step,
            "aria-labelledby": labelId,
          }}
        />
      </Grid2>
    </>
  );
});
