import { Button } from "@mui/material";
import { useContext, useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useRegisterSW } from "virtual:pwa-register/react";
import { ToastQueueContext } from "../components/ToastQueue";

const refreshInterval = import.meta.env.DEV ? 1000 : 1000 * 60 * 60;

export function useUpdateAvailableToast() {
  const [hasToastBeenShown, setHasToastBeenShown] = useState(false);
  const {
    needRefresh: [needRefresh],
    updateServiceWorker,
  } = useRegisterSW({
    onRegisteredSW(_scriptUrl, registration) {
      setInterval(() => {
        registration?.update();
      }, refreshInterval);
      console.log("service worker registered", registration);
    },
    onRegisterError(error) {
      console.error("service worker registration error", error);
    },
  });
  const { t } = useTranslation();
  const toasts = useContext(ToastQueueContext);

  // Only show toast once
  useEffect(() => {
    setHasToastBeenShown(
      (hasToastBeenShown) => hasToastBeenShown || needRefresh
    );
    if (needRefresh && !hasToastBeenShown) {
      toasts.addToast({
        message: t("application.newVersionAvailable"),
        action: (
          <Button onClick={() => updateServiceWorker(true)}>
            {t("application.update")}
          </Button>
        ),
      });
    }
  }, [needRefresh, hasToastBeenShown, toasts, t, updateServiceWorker]);
}
