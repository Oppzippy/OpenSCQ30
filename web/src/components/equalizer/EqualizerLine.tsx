import React from "react";
import { useTranslation } from "react-i18next";

export const EqualizerLine = React.memo(function ({
  values,
}: {
  values: number[];
}) {
  const width = 80;
  const height = 20;
  const padding = 2;
  const getX = (index: number) => (index / values.length) * width;
  // 12 is the minimum equalizer value
  // 0,0 is the top left, but we want 0,0 to be bottom left, so invert the height
  const getY = (value: number) => height - ((value + 12) / 24) * height;

  const { t } = useTranslation();

  return (
    <svg
      viewBox={`${-padding} ${-padding} ${width + padding} ${height + padding}`}
      style={{ height: "1em" }}
      aria-label={values
        .map(
          (value, index) =>
            `${t("equalizer.hz", {
              replace: { hz: Math.pow(2, index) * 100 },
            })}: ${value} dB`,
        )
        .join(", ")}
    >
      <polyline
        fill="none"
        stroke="currentColor"
        strokeWidth={3}
        strokeOpacity={0.4}
        strokeLinecap="round"
        strokeLinejoin="round"
        points={values
          .map((value, index) => `${getX(index)},${getY(value)}`)
          .join(" ")}
      />
    </svg>
  );
});
