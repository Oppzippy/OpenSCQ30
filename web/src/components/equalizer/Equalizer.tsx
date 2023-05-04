import { Divider, Stack } from "@mui/material";
import { VolumeSlider } from "./VolumeSlider";

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
          <VolumeSlider
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
