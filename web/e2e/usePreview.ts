import { test } from "@playwright/test";
import { PreviewServer, preview } from "vite";

export function usePreview(): () => string {
  let address = "";
  let server: PreviewServer;

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

  return () => address;
}
