import { Divider, Stack } from "@mui/material";
import { EqualizerSlider } from "./EqualizerSlider";

type Props = {
  disabled?: boolean;
  values: number[];
  onValueChange: (index: number, newValue: number) => void;
};

export function Equalizer(props: Props) {
  return (
    <Stack spacing={0.2} divider={<Divider />}>
      {props.values.map((value, index) => {
        return (
          <EqualizerSlider
            disabled={props.disabled}
            key={index}
            value={value}
            onValueChange={(newValue) => props.onValueChange(index, newValue)}
          />
        );
      })}
    </Stack>
  );
}
