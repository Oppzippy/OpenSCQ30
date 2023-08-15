import { ValueError } from "@sinclair/typebox/errors";

export class DeviceStateValidationError extends Error {
  state: unknown;
  validationErrors: ValueError[];

  public constructor(state: unknown, validationErrors: ValueError[]) {
    // TODO replace this with { cause: ... } when this issue is closed
    // https://bugs.chromium.org/p/chromium/issues/detail?id=1211260
    super(
      `=====State=====\n${JSON.stringify(
        state,
      )}\n=====Validation Errors=====\n${JSON.stringify(validationErrors)}`,
    );
    this.state = state;
    this.name = "DeviceStateValidationError";
    this.validationErrors = validationErrors;
  }
}
