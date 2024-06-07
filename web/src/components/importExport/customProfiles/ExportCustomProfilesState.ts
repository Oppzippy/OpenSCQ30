import { CustomEqualizerProfile } from "../../../storage/db";

export type ExportCustomProfilesState =
  | ProfileSelectionState
  | CopyToClipboardState;

export function createExportCustomProfilesState(
  profiles: readonly CustomEqualizerProfile[],
): ExportCustomProfilesState {
  return {
    type: "profileSelection",
    profiles,
    selection: new Array(profiles.length).fill(false),
  };
}

export interface ProfileSelectionState {
  type: "profileSelection";
  profiles: readonly CustomEqualizerProfile[];
  selection: readonly boolean[];
}

export interface CopyToClipboardState {
  type: "copyToClipboard";
  profileString: string;
}

export function nextState(
  state: ExportCustomProfilesState,
): ExportCustomProfilesState {
  if (state.type == "profileSelection") {
    return {
      type: "copyToClipboard",
      profileString: JSON.stringify(
        state.profiles
          .filter((_, i) => state.selection[i])
          .map((profile) => ({
            name: profile.name,
            volumeAdjustments: profile.values,
          })),
      ),
    };
  }
  return state;
}

export function isLastState(state: ExportCustomProfilesState) {
  return state.type == "copyToClipboard";
}
