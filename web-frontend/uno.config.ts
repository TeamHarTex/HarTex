import { defineConfig } from "unocss";

import presetIcons from "@unocss/preset-icons";
import presetUno from "@unocss/preset-uno";
import presetWebFonts from "@unocss/preset-web-fonts";
import transformerDirectives from "@unocss/transformer-directives";

export default defineConfig({
  theme: {
    colors: {
      primary: "#401d19",
      secondary: "#FF682C",
      tertiary: "#EDD0C6",
    },
  },
  presets: [
    presetIcons(),
    presetUno(),
    presetWebFonts({
      provider: "fontshare",
      fonts: {
        sans: "Satoshi",
      },
    }),
  ],
  transformers: [transformerDirectives()],
});
