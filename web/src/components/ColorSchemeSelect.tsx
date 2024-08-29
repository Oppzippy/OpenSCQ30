import { DarkMode, LightMode } from "@mui/icons-material";
import { IconButton, Menu, MenuItem, useColorScheme } from "@mui/material";
import { useState } from "react";
import { useTranslation } from "react-i18next";

export function ColorSchemeSelect() {
  const { t } = useTranslation();
  const { mode, setMode, systemMode } = useColorScheme();
  const currentScheme = mode == "system" ? systemMode : mode;

  const [anchorEl, setAnchorEl] = useState<HTMLElement | null>(null);
  const isOpen = !!anchorEl;

  function close() {
    setAnchorEl(null);
  }

  const options = [
    { key: "system", translation: t("colorScheme.system") },
    { key: "light", translation: t("colorScheme.light") },
    { key: "dark", translation: t("colorScheme.dark") },
  ] as const;

  function selectItem(key: (typeof options)[number]["key"]) {
    setMode(key);
    close();
  }

  return (
    <div>
      <IconButton
        id="color-scheme-button"
        aria-label={t("colorScheme.colorScheme")}
        onClick={(event) => setAnchorEl(event.currentTarget)}
      >
        {currentScheme == "dark" ? <LightMode /> : <DarkMode />}
      </IconButton>
      <Menu
        anchorEl={anchorEl}
        open={isOpen}
        onClose={close}
        MenuListProps={{
          "aria-labelledby": "color-scheme-button",
        }}
      >
        {options.map((option, i) => (
          <MenuItem
            key={i}
            onClick={() => selectItem(option.key)}
            selected={mode == option.key}
          >
            {option.translation}
          </MenuItem>
        ))}
      </Menu>
    </div>
  );
}
