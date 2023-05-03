import { Close } from "@mui/icons-material";
import { IconButton, Snackbar, SnackbarCloseReason } from "@mui/material";
import React, {
  PropsWithChildren,
  SyntheticEvent,
  createContext,
  useState,
} from "react";

export type Toast = {
  message: string;
  action?: React.ReactNode;
};
export const ToastQueueContext = createContext({
  toasts: [] as Toast[],
  // eslint-disable-next-line @typescript-eslint/no-unused-vars, @typescript-eslint/no-empty-function
  addToast(_toast: Toast) {},
});

export function ToastQueue({ children }: PropsWithChildren) {
  const [toasts, setToasts] = useState<Toast[]>([]);
  const value = {
    toasts,
    addToast(toast: Toast) {
      setToasts([...toasts, toast]);
    },
  };

  function handleClose(
    _event: Event | SyntheticEvent,
    reason: SnackbarCloseReason
  ) {
    if (reason == "escapeKeyDown" || reason == "timeout") {
      popToast();
    }
  }

  function popToast() {
    setToasts(toasts.slice(1));
  }

  return (
    <ToastQueueContext.Provider value={value}>
      {children}
      {toasts.length == 0 ? undefined : (
        <Snackbar
          open={true}
          message={toasts[0]?.message}
          onClose={handleClose}
          anchorOrigin={{
            horizontal: "right",
            vertical: "bottom",
          }}
          action={
            <>
              {toasts[0].action}
              <IconButton
                size="small"
                aria-label="close"
                color="inherit"
                onClick={() => popToast()}
              >
                <Close fontSize="small" />
              </IconButton>
            </>
          }
        />
      )}
    </ToastQueueContext.Provider>
  );
}
