import { Static, TSchema, Type } from "@sinclair/typebox";
import { TypeCompiler } from "@sinclair/typebox/compiler";

const Nullable = <T extends TSchema>(schema: T) =>
  Type.Union([schema, Type.Null()]);

const twsStatusSchema = Type.Object({
  isConnected: Type.Boolean(),
  hostDevice: Type.Union([Type.Literal("left"), Type.Literal("right")]),
});
export type TwsStatus = Static<typeof twsStatusSchema>;

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

const soundModesTypeTwoSchema = Type.Object({
  ambientSoundMode: Type.Union([
    Type.Literal("noiseCanceling"),
    Type.Literal("transparency"),
    Type.Literal("normal"),
  ]),
  transparencyMode: Type.Union([
    Type.Literal("fullyTransparent"),
    Type.Literal("vocalMode"),
  ]),
  adaptiveNoiseCanceling: Type.Union([
    Type.Literal("lowNoise"),
    Type.Literal("mediumNoise"),
    Type.Literal("highNoise"),
  ]),
  manualNoiseCanceling: Type.Union([
    Type.Literal("weak"),
    Type.Literal("moderate"),
    Type.Literal("strong"),
  ]),
  noiseCancelingMode: Type.Union([
    Type.Literal("adaptive"),
    Type.Literal("manual"),
  ]),
  windNoiseSuppression: Type.Boolean(),
  noiseCancelingAdaptiveSensitivityLevel: Type.Number(),
});
export type SoundModesTypeTwo = Static<typeof soundModesTypeTwoSchema>;

const ambientSoundModeCycleSchema = Type.Object({
  noiseCancelingMode: Type.Boolean(),
  transparencyMode: Type.Boolean(),
  normalMode: Type.Boolean(),
});
export type AmbientSoundModeCycle = Static<typeof ambientSoundModeCycleSchema>;

const equalizerConfigurationSchema = Type.Object({
  presetProfile: Type.Union([
    Type.Null(),
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
  volumeAdjustments: Type.Array(Type.Number()),
});
export type EqualizerConfiguration = Static<
  typeof equalizerConfigurationSchema
>;
export type PresetEqualizerProfile = NonNullable<
  EqualizerConfiguration["presetProfile"]
>;

const buttonActionSchema = Type.Union([
  Type.Literal("volumeUp"),
  Type.Literal("volumeDown"),
  Type.Literal("previousSong"),
  Type.Literal("nextSong"),
  Type.Literal("ambientSoundMode"),
  Type.Literal("voiceAssistant"),
  Type.Literal("playPause"),
  Type.Literal("gameMode"),
]);
export type ButtonAction = Static<typeof buttonActionSchema>;

const twsButtonActionSchema = Type.Object({
  twsConnectedAction: buttonActionSchema,
  twsDisconnectedAction: buttonActionSchema,
  isEnabled: Type.Boolean(),
});
const noTwsButtonActionSchema = Type.Object({
  action: buttonActionSchema,
  isEnabled: Type.Boolean(),
});

const firmwareVersionSchema = Type.Object({
  major: Type.Number({ minimum: 0 }),
  minor: Type.Number({ minimum: 0 }),
});

const deviceFeaturesSchema = Type.Object({
  soundMode: Nullable(
    Type.Object({
      noiseCancelingModeType: Type.Union([
        Type.Literal("none"),
        Type.Literal("basic"),
        Type.Literal("custom"),
      ]),
      transparencyModeType: Type.Union([
        Type.Literal("basic"),
        Type.Literal("custom"),
      ]),
    }),
  ),
  hasHearId: Type.Boolean(),
  numEqualizerChannels: Type.Number({ minimum: 0 }),
  numEqualizerBands: Type.Number({ minimum: 0 }),
  hasDynamicRangeCompression: Type.Boolean(),
  hasCustomButtonModel: Type.Boolean(),
  hasWearDetection: Type.Boolean(),
  hasTouchTone: Type.Boolean(),
  hasAutoPowerOff: Type.Boolean(),
  hasAmbientSoundModeCycle: Type.Boolean(),
  dynamicRangeCompressionMinFirmwareVersion: Nullable(firmwareVersionSchema),
});
export type DeviceFeatures = Static<typeof deviceFeaturesSchema>;

const customButtonModelSchema = Type.Object({
  leftSingleClick: noTwsButtonActionSchema,
  leftDoubleClick: twsButtonActionSchema,
  leftLongPress: twsButtonActionSchema,
  rightSingleClick: noTwsButtonActionSchema,
  rightDoubleClick: twsButtonActionSchema,
  rightLongPress: twsButtonActionSchema,
});
export type CustomButtonModel = Static<typeof customButtonModelSchema>;

const deviceStateSchema = Type.Object({
  deviceFeatures: deviceFeaturesSchema,
  twsStatus: Nullable(twsStatusSchema),
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
  soundModes: Nullable(soundModesSchema),
  soundModesTypeTwo: Nullable(soundModesTypeTwoSchema),
  ageRange: Nullable(Type.Number()),
  gender: Nullable(Type.Number()),
  hearId: Nullable(
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
        customVolumeAdjustments: Nullable(
          Type.Object({
            left: Type.Array(Type.Number()),
            right: Type.Array(Type.Number()),
          }),
        ),
      }),
    ]),
  ),
  firmwareVersion: Nullable(firmwareVersionSchema),
  customButtonModel: Nullable(customButtonModelSchema),
  serialNumber: Nullable(Type.String()),
  ambientSoundModeCycle: Nullable(ambientSoundModeCycleSchema),
});
export type DeviceState = Static<typeof deviceStateSchema>;
export const DeviceStateValidator = TypeCompiler.Compile(deviceStateSchema);
