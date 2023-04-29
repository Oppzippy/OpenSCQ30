import { test, expect } from "@playwright/test";
import { PreviewServer, preview } from "vite";

test.describe("homepage", () => {
  let server: PreviewServer;
  let address: string;

  test.beforeAll(async () => {
    server = await preview({ preview: { port: 0 } });
    const addressInfo = server.httpServer.address();
    if (!addressInfo || typeof addressInfo === "string") {
      throw new Error(`addressInfo is ${addressInfo}, expected an object`);
    }
    if (addressInfo.family == "IPv4") {
      address = `http://${addressInfo.address}:${addressInfo.port}`;
    } else {
      address = `http://[${addressInfo.address}]:${addressInfo.port}`;
    }
  });
  test.afterAll(async () => {
    await new Promise((resolve, reject) => {
      server.httpServer.close((err) => {
        if (err) reject(err);
        else resolve(true);
      });
    });
  });

  test("has title", async ({ page }) => {
    await page.goto(address);

    // Expect a title "to contain" a substring.
    await expect(page).toHaveTitle(/OpenSCQ30/);
  });
});
