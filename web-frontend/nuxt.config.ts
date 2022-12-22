export default defineNuxtConfig({
  modules: ["@nuxtjs/google-fonts", "@nuxtjs/tailwindcss"],
  typescript: {
    strict: true,
    typeCheck: true,
  },
  googleFonts: {
    families: {
      Lato: [400, 700],
    },
  },
});
