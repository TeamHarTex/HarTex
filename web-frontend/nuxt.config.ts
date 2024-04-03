export default defineNuxtConfig({
  app: {
    head: {
      title: "HarTex",
    },
  },
  modules: ["@hypernym/nuxt-gsap", "@unocss/nuxt", "@vueuse/nuxt"],
  gsap: {
    extraPlugins: {
      scrollTrigger: true
    }
  },
  devtools: {
    timeline: {
      enabled: true
    }
  }
});
