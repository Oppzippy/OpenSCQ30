import { CustomEqualizerProfile, db } from "./db";

export async function upsertCustomEqualizerProfile(
  profile: CustomEqualizerProfile,
) {
  await db.transaction("readwrite", db.customEqualizerProfiles, async () => {
    const existing = await db.customEqualizerProfiles
      .where("name")
      .equals(profile.name)
      .or("values")
      .equals(profile.values)
      .first();
    if (existing) {
      await db.customEqualizerProfiles.update(existing, { ...profile });
    } else {
      await db.customEqualizerProfiles.add(profile);
    }
  });
}

export async function upsertCustomEqualizerProfiles(
  profiles: CustomEqualizerProfile[],
) {
  await db.transaction("readwrite", db.customEqualizerProfiles, async () => {
    for (const profile of profiles) {
      const existing = await db.customEqualizerProfiles
        .where("name")
        .equals(profile.name)
        .or("values")
        .equals(profile.values)
        .first();
      if (existing) {
        await db.customEqualizerProfiles.update(existing, { ...profile });
      } else {
        await db.customEqualizerProfiles.add(profile);
      }
    }
  });
}

export async function insertCustomEqualizerProfilesRenameDuplicates(
  profiles: CustomEqualizerProfile[],
) {
  await db.transaction("readwrite", db.customEqualizerProfiles, async () => {
    for (const profile of profiles) {
      const existingValues = await db.customEqualizerProfiles
        .where("values")
        .equals(profile.values)
        .first();
      if (existingValues) {
        continue;
      }
      const existing = await db.customEqualizerProfiles
        .where("name")
        .equals(profile.name)
        .first();
      if (existing) {
        for (let i = 2; i < 100; i++) {
          const newName = `${profile.name} (${i})`;
          if (
            !(await db.customEqualizerProfiles
              .where("name")
              .equals(newName)
              .first())
          ) {
            await db.customEqualizerProfiles.add({ ...profile, name: newName });
            break;
          }
        }
      } else {
        await db.customEqualizerProfiles.add(profile);
      }
    }
  });
}
