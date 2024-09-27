import { Box, Typography } from "@mui/material";
import { ToggleButtonRow, ToggleButtonValues } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React from "react";
import { SoundModesTypeTwo } from "../../libTypes/DeviceState";
import { useFilterNulls } from "../../hooks/useFilterNulls";

interface Props {
  value: SoundModesTypeTwo["noiseCancelingMode"];
  onValueChanged: (newValue: SoundModesTypeTwo["noiseCancelingMode"]) => void;
}

export const NoiseCancelingModeSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const values: ToggleButtonValues<SoundModesTypeTwo["noiseCancelingMode"]> = [
    {
      value: "adaptive",
      label: t("noiseCancelingMode.adaptive"),
    },
    {
      value: "manual",
      label: t("noiseCancelingMode.manual"),
    },
  ];

  return (
    <Box>
      <Typography>{t("soundModes.noiseCancelingMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={values}
      />
    </Box>
  );
});
