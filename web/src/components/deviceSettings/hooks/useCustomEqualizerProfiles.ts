import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../../../storage/db";

export function useCustomEqualizerProfiles() {
  return useLiveQuery(() => db.customEqualizerProfiles.toArray()) ?? [];
}
