import { Stack } from "@mui/material";
import { EqualizerLine } from "./EqualizerLine";
import React from "react";

type Props = {
  name: string;
  volumeAdjustments: ReadonlyArray<number>;
};

export const ProfileRow = React.memo(function ({
  name,
  volumeAdjustments,
}: Props) {
  return (
    <Stack
      direction="row"
      justifyContent="space-between"
      alignItems="center"
      width="100%"
    >
      {name}
      <EqualizerLine volumeAdjustments={volumeAdjustments} />
    </Stack>
  );
});
