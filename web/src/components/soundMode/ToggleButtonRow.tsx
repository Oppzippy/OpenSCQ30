import { ToggleButton, ToggleButtonGroup } from "@mui/material";

type Props<T> = {
  value: T;
  values: {
    value: T;
    displayText: string;
  }[];
  onValueChanged: (newValue: T) => void;
};

export function ToggleButtonRow<T extends NonNullable<unknown>>(
  props: Props<T>
) {
  return (
    <ToggleButtonGroup
      exclusive
      value={props.value}
      onChange={(_, value) => {
        props.onValueChanged(value);
      }}
      sx={{
        display: "flex",
      }}
    >
      {props.values.map(({ value, displayText }, index) => {
        return (
          <ToggleButton value={value} sx={{ flexBasis: "100%" }} key={index}>
            {displayText}
          </ToggleButton>
        );
      })}
    </ToggleButtonGroup>
  );
}
