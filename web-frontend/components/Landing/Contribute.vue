<template>
  <section class="contribute">
    <h2>Want to contribute to or support HarTex?</h2>
    <div class="whitespace"></div>
    <div class="links">
      <NuxtLink class="primary-btn" to="https://github.com/TeamHarTex/HarTex" target="_blank">
        GitHub
      </NuxtLink>
      <NuxtLink class="primary-btn" to="https://opencollective.com/HarTex" target="_blank">
        Open Collective
      </NuxtLink>
    </div>
  </section>
</template>

<style scoped lang="postcss">
.contribute {
  @apply flex flex-col items-center justify-center;

  h2 {
    @apply text-center text-2xl md:text-3xl 5xl:text-5xl;
  }

  .links {
    @apply flex flex-col md:flex-row;

    a {
      @apply my-4 md:my-0;
    }

    a:first-child {
      @apply md:ml-0;
    }

    a:last-child {
      @apply mb-0 md:mr-0;
    }
  }
}
</style>