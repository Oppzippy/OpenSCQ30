import { cleanup } from "@testing-library/react";
import { Dispatch, SetStateAction, useState } from "react";
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
  vi.mock("../src/storage/db", () => {
    return {
      db: {
        customEqualizerProfiles: {
          toArray: () => undefined,
        },
      },
    };
  });
  vi.mock("dexie-react-hooks", () => {
    return {
      useLiveQuery: () => undefined,
    };
  });
});

afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});
