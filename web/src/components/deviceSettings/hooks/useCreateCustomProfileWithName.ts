import { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { useToastErrorHandler } from "../../../hooks/useToastErrorHandler";
import { upsertCustomEqualizerProfile } from "../../../storage/customEqualizerProfiles";

export function useCreateCustomProfileWithName(
  fractionalEqualizerVolumes: number[]
) {
  const { t } = useTranslation();
  const errorHandler = useToastErrorHandler(
    t("errors.failedToCreateCustomProfile")
  );
  return useCallback(
    (name: string) => {
      upsertCustomEqualizerProfile({
        name,
        values: fractionalEqualizerVolumes,
      }).catch(errorHandler);
    },
    [errorHandler, fractionalEqualizerVolumes]
  );
}
