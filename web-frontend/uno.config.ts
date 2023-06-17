import { defineConfig } from 'unocss';

import presetUno from '@unocss/preset-uno';
import presetWebFonts from '@unocss/preset-web-fonts';
import transformerDirectives from "@unocss/transformer-directives";

export default defineConfig({
  presets: [
    presetUno(),
    presetWebFonts({
      provider: 'fontshare',
      fonts: {
        sans: 'Satoshi',
      },
    }),
  ],
  transformers: [
    transformerDirectives(),
  ],
  theme: {
    colors: {
      mainBackground: '#0F1020',
      mainBackgroundSecondary: '#252852',
      mainPrimary: '#29658A',
      mainSecondary: '#474866',
      mainText: '#DDDDDD',
    }
  },
})