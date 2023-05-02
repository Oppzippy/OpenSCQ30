import { cleanup } from "@testing-library/react";
import { afterEach, beforeEach, vi } from "vitest";

beforeEach(() => {
  vi.mock("react-i18next", async () => {
    const actual: object = await vi.importActual("react-i18next");
    const t = (str: string) => {
      return str;
    };
    return {
      ...actual,
      Trans: ({ i18nKey }: { i18nKey: string }) => i18nKey,
      // this mock makes sure any components using the translate hook can use it without a warning being shown
      useTranslation: () => {
        return {
          t,
          i18n: {
            changeLanguage: () => {
              return Promise.resolve();
            },
          },
        };
      },
    };
  });
});

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});
