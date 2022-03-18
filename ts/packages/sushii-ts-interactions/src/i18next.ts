import i18next from "i18next";
import Backend from "i18next-fs-backend";
import log from "./logger";

export default async function initI18next(): Promise<void> {
  await i18next.use(Backend).init(
    {
      fallbackLng: "en",
      ns: ["commands"],
      defaultNS: "commands",
      backend: {
        loadPath: "./locales/{{lng}}/{{ns}}.json",
      },
      saveMissing: true,
    },
    (err) => {
      if (err) {
        log.error(err, "failed to init i18next");
        process.exit(1);
      }

      log.info("i18next initialized");
    }
  );

  i18next.on("missingKey", (lngs, namespace, key, res) => {
    log.warn(
      "missing translation key: %s:%s:%s: %s",
      lngs.join(","),
      namespace,
      key,
      res
    );
  });
}
