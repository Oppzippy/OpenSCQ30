import { test, expect } from "@playwright/test";
import { usePreview } from "./usePreview.ts";

test.describe("homepage", () => {
  const getAddress = usePreview();

  test("has title", async ({ page }) => {
    await page.goto(getAddress());

    // Expect a title "to contain" a substring.
    await expect(page).toHaveTitle(/OpenSCQ30/);
  });
});
