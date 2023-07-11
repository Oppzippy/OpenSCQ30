import { Button } from "@mui/material";
import { useCallback, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useRegisterSW } from "virtual:pwa-register/react";
import { useToasts } from "./useToasts";

const refreshInterval = import.meta.env.DEV ? 1000 : 1000 * 60 * 60;

export function useUpdateAvailableToast() {
  const { t } = useTranslation();
  const toasts = useToasts();
  const [hasToastBeenShown, setHasToastBeenShown] = useState(false);
  const {
    needRefresh: [needRefresh],
    updateServiceWorker,
  } = useRegisterSW({
    onRegisteredSW(_scriptUrl, registration) {
      setInterval(() => {
        registration?.update().catch(console.error);
      }, refreshInterval);
      console.log("service worker registered", registration);
    },
    onRegisterError(error) {
      console.error("service worker registration error", error);
    },
  });
  const update = useCallback(() => {
    updateServiceWorker().catch((err) => {
      console.error(err);
      toasts.addToast({ message: t("application.updateFailed") });
    });
  }, [t, toasts, updateServiceWorker]);

  // Only show toast once
  useEffect(() => {
    setHasToastBeenShown(
      (hasToastBeenShown) => hasToastBeenShown || needRefresh,
    );
    if (needRefresh && !hasToastBeenShown) {
      toasts.addToast({
        message: t("application.newVersionAvailable"),
        action: <Button onClick={update}>{t("application.update")}</Button>,
      });
    }
  }, [hasToastBeenShown, needRefresh, t, toasts, update]);
}
