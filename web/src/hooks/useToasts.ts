import { useContext } from "react";
import { ToastQueueContext } from "../components/ToastQueue";

export function useToasts() {
  return useContext(ToastQueueContext);
}
