import { render } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it, vitest } from "vitest";
import { ExportCustomProfileDialog } from "../../../../src/components/importExport/exportCustomProfiles/ExportCustomProfilesDialog";
import { ExportCustomProfilesState } from "../../../../src/components/importExport/exportCustomProfiles/ExportCustomProfilesState";

describe("ExportCustomProfilesDialog", () => {
  let user: ReturnType<typeof userEvent.setup>;
  beforeEach(() => {
    user = userEvent.setup();
  });

  [
    {
      testName: "no items",
      selection: [false, false],
      buttonText: "selectAll",
    },
    {
      testName: "some items",
      selection: [false, true],
      buttonText: "selectAll",
    },
    {
      testName: "all items",
      selection: [true, true],
      buttonText: "deselectAll",
    },
  ].forEach(({ testName, selection, buttonText }) => {
    it(`should have a ${buttonText} button if ${testName} items are selected`, () => {
      const renderResult = render(
        <ExportCustomProfileDialog
          isOpen={true}
          // eslint-disable-next-line @typescript-eslint/no-empty-function
          onClose={vitest.fn()}
          state={{
            type: "profileSelection",
            profiles: [
              { name: "Profile 1", values: [] },
              { name: "Profile 2", values: [] },
            ],
            selection,
          }}
          onStateChange={vitest.fn()}
        />,
      );
      const selectAllButton = renderResult.getByText(
        `application.${buttonText}`,
      );
      expect(selectAllButton).toBeDefined();
    });
  });

  it("should enable all items when select all is clicked", async () => {
    const state: ExportCustomProfilesState = {
      type: "profileSelection",
      profiles: [
        { name: "Profile 1", values: [] },
        { name: "Profile 2", values: [] },
      ],
      selection: [true, false],
    };
    const onStateChange = vitest.fn();
    const renderResult = render(
      <ExportCustomProfileDialog
        isOpen={true}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onClose={vitest.fn()}
        state={state}
        onStateChange={onStateChange}
      />,
    );
    const selectAllButton = renderResult.getByText(`application.selectAll`);
    await user.click(selectAllButton);
    expect(onStateChange).toBeCalledWith({
      ...state,
      selection: [true, true],
    });
  });

  it("should disable all items when select all is clicked", async () => {
    const state: ExportCustomProfilesState = {
      type: "profileSelection",
      profiles: [
        { name: "Profile 1", values: [] },
        { name: "Profile 2", values: [] },
      ],
      selection: [true, true],
    };
    const onStateChange = vitest.fn();
    const renderResult = render(
      <ExportCustomProfileDialog
        isOpen={true}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onClose={vitest.fn()}
        state={state}
        onStateChange={onStateChange}
      />,
    );
    const deselectAllButton = renderResult.getByText(`application.deselectAll`);
    await user.click(deselectAllButton);
    expect(onStateChange).toBeCalledWith({
      ...state,
      selection: [false, false],
    });
  });

  it("should advance to the copy screen when next is clicked", async () => {
    const state: ExportCustomProfilesState = {
      type: "profileSelection",
      profiles: [
        { name: "Profile 1", values: [1, 2, 3, 4] },
        { name: "Profile 2", values: [5, 6, 7, 8] },
        { name: "Profile 3", values: [9, 10, 11, 12] },
      ],
      selection: [false, true, true],
    };
    const onStateChange = vitest.fn();
    const renderResult = render(
      <ExportCustomProfileDialog
        isOpen={true}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onClose={vitest.fn()}
        state={state}
        onStateChange={onStateChange}
      />,
    );
    const nextButton = renderResult.getByText(`application.next`);
    await user.click(nextButton);
    expect(onStateChange).toBeCalledWith({
      type: "copyToClipboard",
      profileString:
        '[{"name":"Profile 2","volumeAdjustments":[5,6,7,8]},{"name":"Profile 3","volumeAdjustments":[9,10,11,12]}]',
    } satisfies ExportCustomProfilesState);
  });

  it("should have copy and done buttons on the last page", async () => {
    const state: ExportCustomProfilesState = {
      type: "copyToClipboard",
      profileString: "[{...}]",
    };
    const renderResult = render(
      <ExportCustomProfileDialog
        isOpen={true}
        // eslint-disable-next-line @typescript-eslint/no-empty-function
        onClose={vitest.fn()}
        state={state}
        onStateChange={vitest.fn()}
      />,
    );
    const copyButton = renderResult.getByText(`application.copyToClipboard`);
    const doneButton = renderResult.getByText(`application.done`);
    expect(copyButton).toBeDefined();
    expect(doneButton).toBeDefined();
  });
});
