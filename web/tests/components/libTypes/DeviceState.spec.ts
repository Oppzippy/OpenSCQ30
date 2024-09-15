import { describe, expect, it } from "vitest";
import { DeviceState } from "../../../src/libTypes/DeviceState";
import { WasmTest } from "../../../wasm/pkg/openscq30_web_wasm";

describe("libTypes", () => {
  it("should serialize/deserialize an object with as many nulls as possible", () => {
    const expected: DeviceState = {
      deviceProfile: {
        soundMode: null,
        hasHearId: false,
        numEqualizerChannels: 0,
        numEqualizerBands: 0,
        hasDynamicRangeCompression: false,
        hasCustomButtonModel: false,
        hasWearDetection: false,
        hasTouchTone: false,
        hasAutoPowerOff: false,
        dynamicRangeCompressionMinFirmwareVersion: null,
        hasAmbientSoundModeCycle: false,
      },
      ageRange: null,
      gender: null,
      battery: {
        type: "singleBattery",
        isCharging: false,
        level: 0,
      },
      customButtonModel: null,
      hearId: null,
      equalizerConfiguration: {
        presetProfile: null,
        volumeAdjustments: [0, 0, 0, 0, 0, 0, 0, 0],
      },
      firmwareVersion: null,
      serialNumber: null,
      soundModes: null,
      soundModesTypeTwo: null,
      ambientSoundModeCycle: null,
    };
    const actual: unknown = JSON.parse(
      WasmTest.deserializeAndReserializeForTests(JSON.stringify(expected)),
    );
    expect(actual).toEqual(expected);
  });
  it("should serialize/deserialize an object with as many fields filled as possible", () => {
    const expected: DeviceState = {
      deviceProfile: {
        soundMode: {
          noiseCancelingModeType: "custom",
          transparencyModeType: "custom",
        },
        hasHearId: true,
        numEqualizerChannels: 2,
        numEqualizerBands: 8,
        hasDynamicRangeCompression: true,
        hasCustomButtonModel: true,
        hasWearDetection: true,
        hasTouchTone: true,
        hasAutoPowerOff: true,
        dynamicRangeCompressionMinFirmwareVersion: {
          major: 2,
          minor: 3,
        },
        hasAmbientSoundModeCycle: true,
      },
      ageRange: 1,
      gender: 2,
      battery: {
        type: "dualBattery",
        left: {
          isCharging: true,
          level: 1,
        },
        right: {
          isCharging: true,
          level: 2,
        },
      },
      customButtonModel: {
        leftDoubleClick: {
          isEnabled: true,
          twsConnectedAction: "nextSong",
          twsDisconnectedAction: "playPause",
        },
        leftLongPress: {
          isEnabled: true,
          twsConnectedAction: "previousSong",
          twsDisconnectedAction: "ambientSoundMode",
        },
        leftSingleClick: {
          isEnabled: true,
          action: "playPause",
        },
        rightDoubleClick: {
          isEnabled: true,
          twsConnectedAction: "voiceAssistant",
          twsDisconnectedAction: "volumeDown",
        },
        rightLongPress: {
          isEnabled: true,
          twsConnectedAction: "volumeDown",
          twsDisconnectedAction: "volumeUp",
        },
        rightSingleClick: {
          isEnabled: false,
          action: "playPause",
        },
      },
      hearId: {
        isEnabled: true,
        hearIdMusicType: 1,
        hearIdType: 2,
        time: 3,
        volumeAdjustments: {
          left: [11, 12, 13, 14, 15, 16, 17, 18],
          right: [21, 22, 23, 24, 25, 26, 27, 28],
        },
        type: "custom",
        customVolumeAdjustments: {
          left: [0, 1, 2, 3, 4, 5, 6, 7],
          right: [7, 6, 5, 4, 3, 2, 1, 0],
        },
      },
      equalizerConfiguration: {
        presetProfile: "Acoustic",
        volumeAdjustments: [0, 0, 0, 0, 0, 0, 0, 0],
      },
      firmwareVersion: {
        major: 1,
        minor: 2,
      },
      serialNumber: "0123456789ABCDEF",
      soundModes: {
        ambientSoundMode: "noiseCanceling",
        noiseCancelingMode: "custom",
        transparencyMode: "fullyTransparent",
        customNoiseCanceling: 5,
      },
      soundModesTypeTwo: {
        adaptiveNoiseCanceling: "highNoise",
        ambientSoundMode: "noiseCanceling",
        detectedWindNoise: true,
        manualNoiseCanceling: "moderate",
        noiseCancelingAdaptiveSensitivityLevel: 1,
        noiseCancelingMode: "adaptive",
        transparencyMode: "fullyTransparent",
        windNoiseSuppression: true,
      },
      ambientSoundModeCycle: {
        noiseCancelingMode: true,
        transparencyMode: true,
        normalMode: true,
      },
    };
    const actual: unknown = JSON.parse(
      WasmTest.deserializeAndReserializeForTests(JSON.stringify(expected)),
    );
    expect(actual).toEqual(expected);
  });
});
