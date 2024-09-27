import { Box, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { useFilterNulls } from "../../hooks/useFilterNulls";
import { SoundModesTypeTwo } from "../../libTypes/DeviceState";
import { ToggleButtonRow, ToggleButtonValues } from "./ToggleButtonRow";

interface Props {
  value: SoundModesTypeTwo["manualNoiseCanceling"];
  onValueChanged: (newValue: SoundModesTypeTwo["manualNoiseCanceling"]) => void;
}

export const ManualNoiseCancelingSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const values: ToggleButtonValues<SoundModesTypeTwo["manualNoiseCanceling"]> =
    [
      {
        value: "weak",
        label: t("manualNoiseCanceling.weak"),
      },
      {
        value: "moderate",
        label: t("manualNoiseCanceling.moderate"),
      },
      {
        value: "strong",
        label: t("manualNoiseCanceling.strong"),
      },
    ];

  return (
    <Box>
      <Typography>{t("soundModes.manualNoiseCanceling")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={values}
      />
    </Box>
  );
});
