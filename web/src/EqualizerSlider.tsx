import { Box, Input, Slider } from "@mui/material";

type Props = {
  disabled?: boolean;
  value: number;
  onValueChange: (newValue: number) => void;
};

export function EqualizerSlider(props: Props) {
  return (
    <Box sx={{ display: "flex", columnGap: "2rem", alignItems: "start" }}>
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
      <Input
        disabled={props.disabled}
        value={props.value}
        onChange={(event) => props.onValueChange(Number(event.target.value))}
        size="small"
        inputProps={{
          min: -0,
          max: 12,
          step: 0.1,
          type: "number",
          "aria-labelledby": "input-slider",
        }}
      />
    </Box>
  );
}
