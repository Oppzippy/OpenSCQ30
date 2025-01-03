import { Page, expect, test } from "@playwright/test";
import { usePreview } from "./usePreview.ts";

test.describe("custom equalizer profiles", () => {
  const getAddress = usePreview();
  test.beforeEach(async ({ page }) => {
    await page.goto(getAddress());
    await page.evaluate(`localStorage.setItem("openscq30:demoMode", "true")`);
    await page.reload();

    // start tests with custom profile selected
    await page.getByText("Select Device", { exact: true }).click();
    await page.getByText("Soundcore Signature").click();
    await page.getByRole("option", { name: "Custom", exact: true }).click();
  });

  async function createCustomProfile(
    page: Page,
    profile: { name: string; values: number[]; alreadyExists?: boolean },
  ) {
    await selectEqualizerValues(page, profile.values);
    await page.getByLabel("Create Custom Profile").click();
    const profileNameInput = page.getByRole("combobox", {
      name: "Profile Name",
    });
    await profileNameInput.type(profile.name);
    // Cancel dropdown with existing profiles to get it out of the way of the create/overwrite button
    await profileNameInput.press("Escape");
    await page
      .locator("button")
      .getByText(profile.alreadyExists ? "Overwrite" : "Create")
      .click();
  }

  async function selectEqualizerValues(page: Page, values: number[]) {
    const inputs = page.locator("input[type='number']");
    for (let i = 0; i < values.length; i++) {
      const value = values[i];
      const input = inputs.nth(i);
      await input.selectText();
      await input.type(value.toString().replace("-", ""));
      if (value < 0) {
        await input.press("Home");
        await input.type("-");
      }
    }
  }

  test("should create a custom profile", async ({ page }) => {
    await createCustomProfile(page, {
      name: "Test Profile",
      values: [0, 0, 0, 0, 0, 0, 0, 0],
    });

    await expect(page.getByText("Test Profile")).toBeVisible();
  });

  test("should have a delete button when a custom profile is selected, and an add button when it is not", async ({
    page,
  }) => {
    await expect(page.getByLabel("Create Custom Profile")).toBeVisible();
    await expect(page.getByLabel("Delete Custom Profile")).not.toBeVisible();
    await createCustomProfile(page, {
      name: "Test Profile",
      values: [0, 0, 0, 0, 0, 0, 0, 0],
    });
    // the create button from the dialog takes some time to fade, so we have to be more specific
    const createButton = page.locator(
      "button[aria-label='Create Custom Profile']:has(> svg)",
    );
    await expect(page.getByLabel("Delete Custom Profile")).toBeVisible();
    await expect(createButton).not.toBeVisible();
  });

  test("should be able to delete profiles", async ({ page }) => {
    await createCustomProfile(page, {
      name: "Test Profile",
      values: [0, 0, 0, 0, 0, 0, 0, 0],
    });
    await expect(page.getByText("Test Profile")).toBeVisible();
    await page.getByLabel("Delete Custom Profile").click();
    await expect(page.getByText("Test Profile")).not.toBeVisible();
  });

  test("should not allow multiple profiles with the same name", async ({
    page,
  }) => {
    await createCustomProfile(page, {
      name: "Profile 1",
      values: [0, 0, 0, 0, 0, 0, 0, 0],
    });
    await createCustomProfile(page, {
      name: "Profile 1",
      values: [1, 0, 0, 0, 0, 0, 0, 0],
      alreadyExists: true,
    });
    await page.getByText("Profile 1").click();
    expect(await page.locator("li").getByText("Profile 1").count()).toEqual(1);
  });

  test("should allow profiles with different names", async ({ page }) => {
    await createCustomProfile(page, {
      name: "Profile 1",
      values: [0, 0, 0, 0, 0, 0, 0, 0],
    });
    await createCustomProfile(page, {
      name: "Profile 2",
      values: [-1, 0, 0, 0, 0, 0, 0, 0],
    });
    await page.getByText("Profile 2").click();
    await expect(page.getByText("Profile 1")).toBeVisible();
    await expect(page.locator("li").getByText("Profile 2")).toBeVisible();
  });

  test("should select profile when changing eq values", async ({ page }) => {
    await createCustomProfile(page, {
      name: "Profile 1",
      values: [0, 0, 0, 0, 0, 0, 0, 0],
    });
    await expect(page.getByText("Profile 1")).toBeVisible();
    await selectEqualizerValues(page, [1, 0, 0, 0, 0, 0, 0, 0]);
    await expect(page.getByText("Profile 1")).not.toBeVisible();
    await selectEqualizerValues(page, [0, 0, 0, 0, 0, 0, 0, 0]);
    await expect(page.getByText("Profile 1")).toBeVisible();
  });
});
