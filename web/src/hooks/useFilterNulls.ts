import { useCallback } from "react";

export function useFilterNulls<V, R>(callback: (value: V) => R) {
  return useCallback(
    (value: V | undefined | null) => {
      if (value != null) {
        callback(value);
      }
    },
    [callback],
  );
}
