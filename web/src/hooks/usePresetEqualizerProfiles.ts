import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import {
  EqualizerConfiguration,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";

export function usePresetEqualizerProfiles() {
  const { t } = useTranslation();
  return useMemo(() => {
    const presetProfiles = [
      {
        name: t("presetEqualizerProfile.soundcoreSignature"),
        id: PresetEqualizerProfile.SoundcoreSignature,
        values: EqualizerConfiguration.fromPresetProfile(
          PresetEqualizerProfile.SoundcoreSignature,
        ).volumeAdjustments.adjustments,
      },
      {
        name: t("presetEqualizerProfile.acoustic"),
        id: PresetEqualizerProfile.Acoustic,
      },
      {
        name: t("presetEqualizerProfile.bassBooster"),
        id: PresetEqualizerProfile.BassBooster,
      },
      {
        name: t("presetEqualizerProfile.bassReducer"),
        id: PresetEqualizerProfile.BassReducer,
      },
      {
        name: t("presetEqualizerProfile.classical"),
        id: PresetEqualizerProfile.Classical,
      },
      {
        name: t("presetEqualizerProfile.podcast"),
        id: PresetEqualizerProfile.Podcast,
      },
      {
        name: t("presetEqualizerProfile.dance"),
        id: PresetEqualizerProfile.Dance,
      },
      {
        name: t("presetEqualizerProfile.deep"),
        id: PresetEqualizerProfile.Deep,
      },
      {
        name: t("presetEqualizerProfile.electronic"),
        id: PresetEqualizerProfile.Electronic,
      },
      {
        name: t("presetEqualizerProfile.flat"),
        id: PresetEqualizerProfile.Flat,
      },
      {
        name: t("presetEqualizerProfile.hipHop"),
        id: PresetEqualizerProfile.HipHop,
      },
      {
        name: t("presetEqualizerProfile.jazz"),
        id: PresetEqualizerProfile.Jazz,
      },
      {
        name: t("presetEqualizerProfile.latin"),
        id: PresetEqualizerProfile.Latin,
      },
      {
        name: t("presetEqualizerProfile.lounge"),
        id: PresetEqualizerProfile.Lounge,
      },
      {
        name: t("presetEqualizerProfile.piano"),
        id: PresetEqualizerProfile.Piano,
      },
      { name: t("presetEqualizerProfile.pop"), id: PresetEqualizerProfile.Pop },
      { name: t("presetEqualizerProfile.rnB"), id: PresetEqualizerProfile.RnB },
      {
        name: t("presetEqualizerProfile.rock"),
        id: PresetEqualizerProfile.Rock,
      },
      {
        name: t("presetEqualizerProfile.smallSpeakers"),
        id: PresetEqualizerProfile.SmallSpeakers,
      },
      {
        name: t("presetEqualizerProfile.spokenWord"),
        id: PresetEqualizerProfile.SpokenWord,
      },
      {
        name: t("presetEqualizerProfile.trebleBooster"),
        id: PresetEqualizerProfile.TrebleBooster,
      },
      {
        name: t("presetEqualizerProfile.trebleReducer"),
        id: PresetEqualizerProfile.TrebleReducer,
      },
    ];
    const presetProfilesWithValues = presetProfiles.map((profile) => ({
      ...profile,
      values: [
        ...EqualizerConfiguration.fromPresetProfile(profile.id)
          .volumeAdjustments.adjustments,
      ].map((value) => value / 10),
    }));
    return presetProfilesWithValues;
  }, [t]);
}
