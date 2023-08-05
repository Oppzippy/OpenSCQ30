import { Static, Type } from "@sinclair/typebox";
import { TypeCompiler } from "@sinclair/typebox/compiler";

const singleBatterySchema = Type.Object({
  isCharging: Type.Boolean(),
  level: Type.Number(),
});
export type SingleBattery = Static<typeof singleBatterySchema>;

const soundModesSchema = Type.Object({
  ambientSoundMode: Type.Union([
    Type.Literal("noiseCanceling"),
    Type.Literal("transparency"),
    Type.Literal("normal"),
  ]),
  noiseCancelingMode: Type.Union([
    Type.Literal("indoor"),
    Type.Literal("outdoor"),
    Type.Literal("transport"),
    Type.Literal("custom"),
  ]),
  transparencyMode: Type.Union([
    Type.Literal("fullyTransparent"),
    Type.Literal("vocalMode"),
  ]),
  customNoiseCanceling: Type.Number(),
});
export type SoundModes = Static<typeof soundModesSchema>;

const equalizerConfigurationSchema = Type.Object({
  presetProfile: Type.Optional(
    Type.Union([
      Type.Literal("SoundcoreSignature"),
      Type.Literal("Acoustic"),
      Type.Literal("BassBooster"),
      Type.Literal("BassReducer"),
      Type.Literal("Classical"),
      Type.Literal("Podcast"),
      Type.Literal("Dance"),
      Type.Literal("Deep"),
      Type.Literal("Electronic"),
      Type.Literal("Flat"),
      Type.Literal("HipHop"),
      Type.Literal("Jazz"),
      Type.Literal("Latin"),
      Type.Literal("Lounge"),
      Type.Literal("Piano"),
      Type.Literal("Pop"),
      Type.Literal("RnB"),
      Type.Literal("Rock"),
      Type.Literal("SmallSpeakers"),
      Type.Literal("SpokenWord"),
      Type.Literal("TrebleBooster"),
      Type.Literal("TrebleReducer"),
    ]),
  ),
  volumeAdjustments: Type.Array(Type.Number()),
});
export type EqualizerConfiguration = Static<
  typeof equalizerConfigurationSchema
>;
export type PresetEqualizerProfile = NonNullable<
  EqualizerConfiguration["presetProfile"]
>;

const deviceStateSchema = Type.Object({
  featureFlags: Type.Number(),
  battery: Type.Union([
    Type.Object({
      type: Type.Literal("singleBattery"),
      ...singleBatterySchema.properties,
    }),
    Type.Object({
      type: Type.Literal("dualBattery"),
      left: singleBatterySchema,
      right: singleBatterySchema,
    }),
  ]),
  equalizerConfiguration: equalizerConfigurationSchema,
  soundModes: Type.Optional(soundModesSchema),
  ageRange: Type.Optional(Type.Number()),
  customHearId: Type.Optional(
    Type.Union([
      Type.Object({
        type: Type.Literal("basic"),
        isEnabled: Type.Boolean(),
        volumeAdjustments: Type.Object({
          left: Type.Array(Type.Number()),
          right: Type.Array(Type.Number()),
        }),
        time: Type.Number(),
      }),
      Type.Object({
        type: Type.Literal("custom"),
        isEnabled: Type.Boolean(),
        volumeAdjustments: Type.Object({
          left: Type.Array(Type.Number()),
          right: Type.Array(Type.Number()),
        }),
        time: Type.Number(),
        hearIdType: Type.Number(),
        hearIdMusicType: Type.Number(),
        customVolumeAdjustments: Type.Optional(
          Type.Object({
            left: Type.Array(Type.Number()),
            right: Type.Array(Type.Number()),
          }),
        ),
      }),
    ]),
  ),
  firmwareVersion: Type.Optional(Type.String()),
  serialNumber: Type.Optional(Type.String()),
});
export type DeviceState = Static<typeof deviceStateSchema>;
export const DeviceStateValidator = TypeCompiler.Compile(deviceStateSchema);
