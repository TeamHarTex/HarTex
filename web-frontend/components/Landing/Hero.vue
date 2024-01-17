<template>
  <header>
    <h1>
      Discord server management,<br/>
      <span>reimagined</span>.
    </h1>
    <NuxtLink class="primary-btn">Add to Discord</NuxtLink>
  </header>
</template>

<style scoped lang="postcss">
header {
  @apply h-[95vh] flex flex-col items-center justify-center;

  h1 {
    @apply text-5xl md:text-6xl 5xl:text-8xl font-500 text-center;

    span {
      @apply text-secondary font-serif;
    }
  }

  .primary-btn {
    @apply mt-20 cursor-not-allowed opacity-50;
  }

  .primary-btn:hover {
    @apply bg-primary text-tertiary;
  }
}
</style>