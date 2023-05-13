import { useTranslation } from "react-i18next";
import { useCallback } from "react";
import { useToastErrorHandler } from "../../../hooks/useToastErrorHandler";
import { useToasts } from "../../../hooks/useToasts";
import { CustomEqualizerProfile, db } from "../../../storage/db";

export function useDeleteCustomProfile() {
  const { t } = useTranslation();
  const errorMessage = t("errors.failedToDeleteCustomProfile");
  const errorHandler = useToastErrorHandler(errorMessage);
  const toasts = useToasts();

  return useCallback(
    (profileToDelete: CustomEqualizerProfile) => {
      if (profileToDelete.id) {
        db.customEqualizerProfiles
          .delete(profileToDelete.id)
          .catch(errorHandler);
      } else {
        toasts.addToast({
          message: errorMessage,
        });
      }
    },
    [errorHandler, errorMessage, toasts]
  );
}
