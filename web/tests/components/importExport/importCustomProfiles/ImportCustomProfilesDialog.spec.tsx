import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it } from "vitest";
import { ImportExport } from "../../../../src/components/importExport/ImportExport";
import {
  fetchCustomEqualizerProfiles,
  upsertCustomEqualizerProfiles,
} from "../../../../src/storage/customEqualizerProfiles";

describe("ImportCustomProfilesDialog", () => {
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    user = userEvent.setup();
  });

  [
    {
      testName: "should import a profile when no current profiles exist",
      overwrite: false,
      preImportProfiles: [],
      postImportProfiles: [{ name: "Test", values: [1, 2, 3, 4, 5, 6, 7, 8] }],
      importString: JSON.stringify([
        { name: "Test", volumeAdjustments: [1, 2, 3, 4, 5, 6, 7, 8] },
      ]),
    },
    {
      testName: "should rename profiles when profiles with the same name exist",
      overwrite: false,
      preImportProfiles: [{ name: "Test", values: [1, 0, 0, 0, 0, 0, 0, 0] }],
      postImportProfiles: [
        { name: "Test", values: [1, 0, 0, 0, 0, 0, 0, 0] },
        { name: "Test (2)", values: [2, 0, 0, 0, 0, 0, 0, 0] },
        { name: "Test (3)", values: [3, 0, 0, 0, 0, 0, 0, 0] },
      ],
      importString: JSON.stringify([
        { name: "Test", volumeAdjustments: [2, 0, 0, 0, 0, 0, 0, 0] },
        { name: "Test", volumeAdjustments: [3, 0, 0, 0, 0, 0, 0, 0] },
      ]),
    },
    {
      testName: "should overwrite existing profiles",
      overwrite: true,
      preImportProfiles: [{ name: "Test", values: [1, 0, 0, 0, 0, 0, 0, 0] }],
      postImportProfiles: [{ name: "Test", values: [2, 0, 0, 0, 0, 0, 0, 0] }],
      importString: JSON.stringify([
        { name: "Test", volumeAdjustments: [2, 0, 0, 0, 0, 0, 0, 0] },
      ]),
    },
  ].forEach(
    ({
      testName,
      overwrite,
      preImportProfiles,
      postImportProfiles,
      importString,
    }) => {
      it(testName, async () => {
        await upsertCustomEqualizerProfiles(preImportProfiles);
        const renderResult = render(<ImportExport />);
        await user.click(
          renderResult.getByText("equalizer.importCustomProfiles"),
        );
        await user.type(
          renderResult.getByLabelText("equalizer.customProfilesJSON"),
          // escape special characters for user.type
          importString.replaceAll("{", "{{").replaceAll("[", "[["),
        );
        await user.click(renderResult.getByText("application.next"));
        const overwriteCheckbox = renderResult.getByRole("checkbox", {
          name: "equalizer.overwriteExistingProfiles",
        }) as HTMLInputElement;
        if (overwriteCheckbox.checked != overwrite) {
          await user.click(overwriteCheckbox);
        }
        expect(overwriteCheckbox.checked).toEqual(overwrite);
        await user.click(renderResult.getByText("application.import"));

        expect(
          (await fetchCustomEqualizerProfiles()).map((profile) => ({
            name: profile.name,
            values: profile.values,
          })),
        ).toEqual(postImportProfiles);
      });
    },
  );
});
