import { Input, Slider, Typography } from "@mui/material";
import Grid2 from "@mui/material/Unstable_Grid2";
import { useTranslation } from "react-i18next";

type Props = {
  hz: number;
  disabled?: boolean;
  value: number;
  onValueChange: (newValue: number) => void;
};

export function VolumeSlider(props: Props) {
  const { t } = useTranslation();
  return (
    <>
      {/* make sure Hz doesn't go on to a second line */}
      <Grid2 xs={3} sm={2}>
        <Typography>
          {props.hz >= 10000
            ? t("equalizer.khz", {
                defaultValue: "{{ hz }} kHz",
                replace: { hz: props.hz / 1000 },
              })
            : t("equalizer.hz", {
                defaultValue: "{{ hz }} Hz",
                replace: { hz: props.hz },
              })}
        </Typography>
      </Grid2>
      <Grid2 xs={7} sm={8}>
        <Slider
          disabled={props.disabled}
          value={props.value}
          min={-12}
          max={12}
          step={0.1}
          valueLabelDisplay="auto"
          marks={[
            { value: -12, label: "-12 dB" },
            { value: 12, label: "12 dB" },
          ]}
          onChange={(_, value) => {
            if (typeof value == "number") {
              props.onValueChange(value);
            } else {
              throw Error(`Expected single number, got number array: ${value}`);
            }
          }}
        />
      </Grid2>
      <Grid2 xs={2}>
        <Input
          disabled={props.disabled}
          value={props.value}
          onChange={(event) => props.onValueChange(Number(event.target.value))}
          size="small"
          inputProps={{
            min: -12,
            max: 12,
            step: 0.1,
            type: "number",
            "aria-labelledby": "input-slider",
          }}
        />
      </Grid2>
    </>
  );
}
