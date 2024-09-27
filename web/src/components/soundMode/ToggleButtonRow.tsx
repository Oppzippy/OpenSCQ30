import { ToggleButton, ToggleButtonGroup } from "@mui/material";

interface Props<T> {
  value: T;
  values: ToggleButtonValues<T>;
  onValueChanged: (newValue: T | undefined) => void;
}

export type ToggleButtonValues<T> = {
  value: T;
  label: string;
}[];

export function ToggleButtonRow<T extends NonNullable<unknown>>(
  props: Props<T>,
) {
  return (
    <ToggleButtonGroup
      exclusive
      value={props.value}
      onChange={(_, value) => {
        props.onValueChanged(value as T | undefined);
      }}
      sx={{
        display: "flex",
      }}
    >
      {props.values.map(({ value, label }, index) => {
        return (
          <ToggleButton value={value} sx={{ flexBasis: "100%" }} key={index}>
            {label}
          </ToggleButton>
        );
      })}
    </ToggleButtonGroup>
  );
}
