import i18n from "i18next";
import LanguageDetector from "i18next-browser-languagedetector";
import { initReactI18next } from "react-i18next";

// TODO figure something out so translations don't have to be individually imported
import enTranslation from "../locales/en/translation.json";
import jaTranslation from "../locales/ja/translation.json";

await i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    debug: import.meta.env.DEV,
    fallbackLng: "en",
    returnEmptyString: false,
    interpolation: {
      escapeValue: false, // escaped by react
    },
    resources: {
      en: {
        translation: enTranslation,
      },
      ja: {
        translation: jaTranslation,
      },
    },
  });
