import { useCallback } from "react";
import { useToasts } from "./useToasts";

export function useToastErrorHandler(message: string) {
  const toasts = useToasts();
  return useCallback(
    (err: Error) => {
      console.error(err);
      toasts.addToast({
        message,
      });
    },
    [message, toasts]
  );
}
