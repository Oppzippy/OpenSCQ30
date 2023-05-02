import i18n from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import HttpApi, { HttpBackendOptions } from "i18next-http-backend";
import { initReactI18next } from "react-i18next";

i18n
  .use(HttpApi)
  .use(LanguageDetector)
  .use(initReactI18next)
  .init<HttpBackendOptions>({
    debug: import.meta.env.DEV,
    fallbackLng: "en",
    returnEmptyString: false,
    interpolation: {
      escapeValue: false, // escaped by react
    },
    backend: {
      loadPath: "./locales/{{lng}}/{{ns}}.json",
    },
  });
