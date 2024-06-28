import { importCustomEqualizerProfiles } from "../../../importExport/CustomEqualizerProfile";
import { CustomEqualizerProfile } from "../../../storage/db";

export type ImportCustomProfilesState = StringInputState | ImportOptionsState;

export function createImportCustomProfilesState(
  existingProfiles: CustomEqualizerProfile[],
): ImportCustomProfilesState {
  return {
    type: "stringInput",
    profileString: "",
    existingProfiles,
  };
}

export interface StringInputState {
  type: "stringInput";
  profileString: string;
  error?: unknown;
  existingProfiles: CustomEqualizerProfile[];
}

export interface ImportOptionsState {
  type: "importOptions";
  profiles: CustomEqualizerProfile[];
  selection: boolean[];
  rename: (string | undefined)[];
  overwrite: boolean;
  existingProfiles: CustomEqualizerProfile[];
}

export interface CustomProfileImportCommand {
  profiles: CustomEqualizerProfile[];
  overwrite: boolean;
}

export function nextState(
  state: ImportCustomProfilesState,
): ImportCustomProfilesState {
  if (state.type == "stringInput") {
    try {
      const profiles = importCustomEqualizerProfiles(state.profileString);
      return {
        type: "importOptions",
        profiles,
        selection: new Array<boolean>(profiles.length).fill(true),
        rename: new Array<string | undefined>(profiles.length),
        overwrite: false,
        existingProfiles: state.existingProfiles,
      };
    } catch (error: unknown) {
      return {
        ...state,
        error,
      };
    }
  }
  return state;
}

export function prepareProfilesForImport(
  state: ImportOptionsState,
): CustomProfileImportCommand {
  const newProfiles = state.profiles
    .map((profile, i) => ({
      ...profile,
      name: state.rename[i] ?? profile.name,
    }))
    .filter((_, index) => state.selection[index]);

  return {
    profiles: newProfiles,
    overwrite: state.overwrite,
  };
}

export function getProfileName(state: ImportOptionsState, index: number) {
  if (state.rename[index] != "") {
    return state.rename[index] ?? state.profiles[index].name;
  } else {
    return state.profiles[index].name;
  }
}

export function toggleProfileSelection(
  state: ImportOptionsState,
  index: number,
) {
  const newSelection = [...state.selection];
  newSelection[index] = !newSelection[index];
  return {
    ...state,
    selection: newSelection,
  };
}

export function renameProfile(
  state: ImportOptionsState,
  index: number,
  newName: string | undefined,
) {
  const newRename = [...state.rename];
  newRename[index] =
    newName != state.profiles[index].name ? newName : undefined;
  return {
    ...state,
    rename: newRename,
  };
}

export function isLastState(
  state: ImportCustomProfilesState,
): state is ImportOptionsState {
  return state.type == "importOptions";
}
