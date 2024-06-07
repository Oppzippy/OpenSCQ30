import { useLiveQuery } from "dexie-react-hooks";
import { CustomEqualizerProfile, db } from "../../../storage/db";

const emptyArray: CustomEqualizerProfile[] = [];

export function useCustomEqualizerProfiles() {
  return useLiveQuery(
    () => db.customEqualizerProfiles.toArray(),
    [],
    emptyArray,
  );
}
