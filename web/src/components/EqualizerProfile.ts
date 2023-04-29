import { PresetEqualizerProfile } from "../../wasm/pkg/openscq30_web_wasm";

export type PresetProfileName = (typeof presetProfiles)[number]["name"];
export const presetProfiles = [
  { name: "SoundcoreSignature", id: PresetEqualizerProfile.SoundcoreSignature },
  { name: "Acoustic", id: PresetEqualizerProfile.Acoustic },
  { name: "BassBooster", id: PresetEqualizerProfile.BassBooster },
  { name: "BassReducer", id: PresetEqualizerProfile.BassReducer },
  { name: "Classical", id: PresetEqualizerProfile.Classical },
  { name: "Podcast", id: PresetEqualizerProfile.Podcast },
  { name: "Dance", id: PresetEqualizerProfile.Dance },
  { name: "Deep", id: PresetEqualizerProfile.Deep },
  { name: "Electronic", id: PresetEqualizerProfile.Electronic },
  { name: "Flat", id: PresetEqualizerProfile.Flat },
  { name: "HipHop", id: PresetEqualizerProfile.HipHop },
  { name: "Jazz", id: PresetEqualizerProfile.Jazz },
  { name: "Latin", id: PresetEqualizerProfile.Latin },
  { name: "Lounge", id: PresetEqualizerProfile.Lounge },
  { name: "Piano", id: PresetEqualizerProfile.Piano },
  { name: "Pop", id: PresetEqualizerProfile.Pop },
  { name: "RnB", id: PresetEqualizerProfile.RnB },
  { name: "Rock", id: PresetEqualizerProfile.Rock },
  { name: "SmallSpeakers", id: PresetEqualizerProfile.SmallSpeakers },
  { name: "SpokenWord", id: PresetEqualizerProfile.SpokenWord },
  { name: "TrebleBooster", id: PresetEqualizerProfile.TrebleBooster },
  { name: "TrebleReducer", id: PresetEqualizerProfile.TrebleReducer },
] as const;
