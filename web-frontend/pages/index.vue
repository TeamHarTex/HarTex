<template>
  <section class="hero">
    <h2>
      The speed of thought<br class="hidden sm:block" />
      Discord management bot.
    </h2>
    <br />
    <p>
      Made with love by the HarTex team.
    </p>
    <br />
    <button>Get Started</button>
  </section>
</template>

<style lang="postcss" scoped>
.hero {
  @apply flex flex-col;
  @apply text-center justify-center items-center;
}

.hero h2 {
  @apply text-4xl sm:text-5xl xl:text-6xl font-bold tracking-tight;
}

.hero p {
  @apply text-sm sm:text-lg text-tertiary;
}
</style>
