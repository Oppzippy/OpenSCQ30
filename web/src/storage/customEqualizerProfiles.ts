import { CustomEqualizerProfile, db } from "./db";

export async function upsertCustomEqualizerProfile(
  profile: CustomEqualizerProfile,
) {
  await db.transaction("readwrite", db.customEqualizerProfiles, async () => {
    const existing = await db.customEqualizerProfiles
      .where("name")
      .equalsIgnoreCase(profile.name)
      .or("values")
      .equals(profile.values)
      .first();
    if (existing) {
      await db.customEqualizerProfiles.update(existing, profile);
    } else {
      await db.customEqualizerProfiles.add(profile);
    }
  });
}
