export default defineNuxtConfig({
  modules: ["@nuxtjs/google-fonts", "nuxt-icon", "@nuxtjs/tailwindcss"],
  typescript: {
    strict: true,
    typeCheck: true,
  },
  googleFonts: {
    families: {
      Inter: [400, 600, 700],
    },
  },
});
