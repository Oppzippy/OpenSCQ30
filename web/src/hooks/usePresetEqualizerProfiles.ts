import { useTranslation } from "react-i18next";
import { PresetEqualizerProfile } from "../../wasm/pkg/openscq30_web_wasm";

export function usePresetEqualizerProfiles() {
  const { t } = useTranslation();
  return [
    {
      name: t("equalizerPresetProfile.soundcoreSignature"),
      id: PresetEqualizerProfile.SoundcoreSignature,
    },
    {
      name: t("equalizerPresetProfile.acoustic"),
      id: PresetEqualizerProfile.Acoustic,
    },
    {
      name: t("equalizerPresetProfile.bassBooster"),
      id: PresetEqualizerProfile.BassBooster,
    },
    {
      name: t("equalizerPresetProfile.bassReducer"),
      id: PresetEqualizerProfile.BassReducer,
    },
    {
      name: t("equalizerPresetProfile.classical"),
      id: PresetEqualizerProfile.Classical,
    },
    {
      name: t("equalizerPresetProfile.podcast"),
      id: PresetEqualizerProfile.Podcast,
    },
    {
      name: t("equalizerPresetProfile.dance"),
      id: PresetEqualizerProfile.Dance,
    },
    {
      name: t("equalizerPresetProfile.deep"),
      id: PresetEqualizerProfile.Deep,
    },
    {
      name: t("equalizerPresetProfile.electronic"),
      id: PresetEqualizerProfile.Electronic,
    },
    {
      name: t("equalizerPresetProfile.flat"),
      id: PresetEqualizerProfile.Flat,
    },
    {
      name: t("equalizerPresetProfile.hipHop"),
      id: PresetEqualizerProfile.HipHop,
    },
    {
      name: t("equalizerPresetProfile.jazz"),
      id: PresetEqualizerProfile.Jazz,
    },
    {
      name: t("equalizerPresetProfile.latin"),
      id: PresetEqualizerProfile.Latin,
    },
    {
      name: t("equalizerPresetProfile.lounge"),
      id: PresetEqualizerProfile.Lounge,
    },
    {
      name: t("equalizerPresetProfile.piano"),
      id: PresetEqualizerProfile.Piano,
    },
    { name: t("equalizerPresetProfile.pop"), id: PresetEqualizerProfile.Pop },
    { name: t("equalizerPresetProfile.rnB"), id: PresetEqualizerProfile.RnB },
    {
      name: t("equalizerPresetProfile.rock"),
      id: PresetEqualizerProfile.Rock,
    },
    {
      name: t("equalizerPresetProfile.smallSpeakers"),
      id: PresetEqualizerProfile.SmallSpeakers,
    },
    {
      name: t("equalizerPresetProfile.spokenWord"),
      id: PresetEqualizerProfile.SpokenWord,
    },
    {
      name: t("equalizerPresetProfile.trebleBooster"),
      id: PresetEqualizerProfile.TrebleBooster,
    },
    {
      name: t("equalizerPresetProfile.trebleReducer"),
      id: PresetEqualizerProfile.TrebleReducer,
    },
  ];
}
