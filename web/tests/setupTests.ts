import "fake-indexeddb/auto";

import { cleanup } from "@testing-library/react";
import { Dispatch, SetStateAction, useState } from "react";
import { afterEach, beforeEach, vi } from "vitest";
import { OpenSCQ30Dexie } from "../src/storage/db";

beforeEach(async () => {
  const db = new OpenSCQ30Dexie();
  await db.delete();

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
  vi.mock("virtual:pwa-register/react", () => {
    let needRefresh: boolean, setNeedRefresh: Dispatch<SetStateAction<boolean>>;
    return {
      get needRefresh() {
        return needRefresh;
      },
      set needRefresh(value: boolean) {
        setNeedRefresh(value);
      },
      useRegisterSW() {
        [needRefresh, setNeedRefresh] = useState(false);
        return {
          needRefresh: [needRefresh, setNeedRefresh],
          offlineReady: [false, vi.fn()],
          updateServiceWorker: vi.fn(),
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
