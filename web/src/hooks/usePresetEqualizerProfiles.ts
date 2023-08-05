import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { EqualizerHelper } from "../../wasm/pkg/openscq30_web_wasm";
import { PresetEqualizerProfile } from "../libTypes/DeviceState";

export function usePresetEqualizerProfiles() {
  const { t } = useTranslation();
  return useMemo(() => {
    const presetProfiles: {
      name: string;
      id: PresetEqualizerProfile;
    }[] = [
      {
        name: t("presetEqualizerProfile.soundcoreSignature"),
        id: "SoundcoreSignature",
      },
      {
        name: t("presetEqualizerProfile.acoustic"),
        id: "Acoustic",
      },
      {
        name: t("presetEqualizerProfile.bassBooster"),
        id: "BassBooster",
      },
      {
        name: t("presetEqualizerProfile.bassReducer"),
        id: "BassReducer",
      },
      {
        name: t("presetEqualizerProfile.classical"),
        id: "Classical",
      },
      {
        name: t("presetEqualizerProfile.podcast"),
        id: "Podcast",
      },
      {
        name: t("presetEqualizerProfile.dance"),
        id: "Dance",
      },
      {
        name: t("presetEqualizerProfile.deep"),
        id: "Deep",
      },
      {
        name: t("presetEqualizerProfile.electronic"),
        id: "Electronic",
      },
      {
        name: t("presetEqualizerProfile.flat"),
        id: "Flat",
      },
      {
        name: t("presetEqualizerProfile.hipHop"),
        id: "HipHop",
      },
      {
        name: t("presetEqualizerProfile.jazz"),
        id: "Jazz",
      },
      {
        name: t("presetEqualizerProfile.latin"),
        id: "Latin",
      },
      {
        name: t("presetEqualizerProfile.lounge"),
        id: "Lounge",
      },
      {
        name: t("presetEqualizerProfile.piano"),
        id: "Piano",
      },
      { name: t("presetEqualizerProfile.pop"), id: "Pop" },
      { name: t("presetEqualizerProfile.rnB"), id: "RnB" },
      {
        name: t("presetEqualizerProfile.rock"),
        id: "Rock",
      },
      {
        name: t("presetEqualizerProfile.smallSpeakers"),
        id: "SmallSpeakers",
      },
      {
        name: t("presetEqualizerProfile.spokenWord"),
        id: "SpokenWord",
      },
      {
        name: t("presetEqualizerProfile.trebleBooster"),
        id: "TrebleBooster",
      },
      {
        name: t("presetEqualizerProfile.trebleReducer"),
        id: "TrebleReducer",
      },
    ];
    const presetProfilesWithValues = presetProfiles.map((profile) => ({
      ...profile,
      values: [
        ...EqualizerHelper.getPresetProfileVolumeAdjustments(profile.id),
      ].map((value) => value / 10),
    }));
    return presetProfilesWithValues;
  }, [t]);
}
