import { defineConfig } from "unocss";

import presetIcons from "@unocss/preset-icons";
import presetUno from "@unocss/preset-uno";
import presetWebFonts from "@unocss/preset-web-fonts";
import transformerDirectives from "@unocss/transformer-directives";

export default defineConfig({
  theme: {
    breakpoints: {
      sm: "640px",
      md: "768px",
      lg: "1024px",
      xl: "1280px",
      "2xl": "1600px",
      "3xl": "1920px",
      "4xl": "2048px",
      "5xl": "2560px",
    },
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
        serif: "Sentient",
      },
    }),
  ],
  transformers: [transformerDirectives()],
});
