import { Type } from "@sinclair/typebox";
import { CustomEqualizerProfile as DBCustomEqualizerProfile } from "../storage/db";
import { TypeCompiler } from "@sinclair/typebox/compiler";

interface CustomEqualizerProfile {
  name: string;
  volumeAdjustments: readonly number[];
}

const customEqualizerProfilesSchema = Type.Array(
  Type.Object({
    name: Type.String(),
    volumeAdjustments: Type.Array(Type.Number()),
  }),
);
const customEqualizerProfilesValidator = TypeCompiler.Compile(
  customEqualizerProfilesSchema,
);

export function exportCustomEqualizerProfiles(
  profiles: readonly DBCustomEqualizerProfile[],
): CustomEqualizerProfile[] {
  return profiles.map((profile) => ({
    name: profile.name,
    volumeAdjustments: profile.values,
  }));
}

export function importCustomEqualizerProfiles(
  profilesString: string,
): DBCustomEqualizerProfile[] {
  const profiles = customEqualizerProfilesValidator.Decode(
    JSON.parse(profilesString),
  );
  return profiles.map((profile) => ({
    name: profile.name,
    values: profile.volumeAdjustments,
  }));
}
